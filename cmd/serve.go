package cmd

import (
	"context"
	"errors"
	"net"
	"sync"
	"sync/atomic"
	"syscall"
	"time"

	"github.com/gofiber/fiber/v3"
	"github.com/redcardinal-io/auth/internal/api"
	"github.com/redcardinal-io/auth/internal/conf"
	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"
	"golang.org/x/sys/unix"
)

var serveCmd = &cobra.Command{
	Use:  "serve",
	Long: "Start the server",
	Run: func(cmd *cobra.Command, args []string) {
		serve(cmd.Context())
	},
}

func serve(ctx context.Context) {
	if err := conf.LoadFile(configFile); err != nil {
		logrus.WithError(err).Fatal("unable to load config")
	}

	if err := conf.LoadDirectory(watchDir); err != nil {
		logrus.WithError(err).Fatal("unable to load config directory")
	}

	config, err := conf.LoadGlobalFromEnv()
	if err != nil {
		logrus.WithError(err).Fatal("unable to load config")
	}

	addr := net.JoinHostPort(config.API.Host, config.API.Port)
	// Create new Fiber app with appropriate configurations
	app := fiber.New(fiber.Config{
		ReadTimeout:  2 * time.Second,
		IdleTimeout:  time.Minute,
		WriteTimeout: time.Minute,
		AppName:      "RCAuth API",
	})
	a := api.NewFiberAPIWithVersion(config)
	a.SetupRoutes(app)
	logrus.WithField("version", a.Version()).Infof("GoTrue API started on: %s", addr)

	log := logrus.WithField("component", "api")
	var wg sync.WaitGroup
	defer wg.Wait()

	// Create an atomic reference to the current app for hot reloading
	var currentApp atomic.Value
	currentApp.Store(app)

	// Setup graceful shutdown
	wg.Add(1)
	go func() {
		defer wg.Done()
		<-ctx.Done()

		shutdownCtx, shutdownCancel := context.WithTimeout(context.Background(), time.Minute)
		defer shutdownCancel()

		// Get the current app and shut it down
		if err := currentApp.Load().(*fiber.App).ShutdownWithContext(shutdownCtx); err != nil &&
			!errors.Is(err, context.Canceled) {
			log.WithError(err).Error("shutdown failed")
		}
	}()

	// Configure listener with SO_REUSEPORT option
	lc := net.ListenConfig{
		Control: func(network, address string, c syscall.RawConn) error {
			var serr error
			if err := c.Control(func(fd uintptr) {
				serr = unix.SetsockoptInt(int(fd), unix.SOL_SOCKET, unix.SO_REUSEPORT, 1)
			}); err != nil {
				return err
			}
			return serr
		},
	}

	listener, err := lc.Listen(ctx, "tcp", addr)
	if err != nil {
		log.WithError(err).Fatal("fiber server listen failed")
	}

	// Start the Fiber server
	if err := app.Listener(listener); err != nil {
		log.WithError(err).Fatal("fiber server serve failed")
	}
}
