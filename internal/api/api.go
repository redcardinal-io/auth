package api

import (
	"regexp"
	"time"

	"github.com/gofiber/fiber/v3"
	"github.com/gofiber/fiber/v3/middleware/logger"
	"github.com/redcardinal-io/auth/internal/conf"
	"github.com/sirupsen/logrus"
	"github.com/supabase/hibp"
)

const (
	audHeaderName  = "X-JWT-AUD"
	defaultVersion = "unknown version"
)

var bearerRegexp = regexp.MustCompile(`^(?:B|b)earer (\S+$)`)

type API struct {
	config       *conf.GlobalConfiguration
	version      string
	overrideTime func() time.Time
	limiterOpts  *LimiterOptions
	hibpClient   *hibp.PwnedClient
}

func (a *API) Version() string {
	return a.version
}

func (a *API) Now() time.Time {
	if a.overrideTime != nil {
		return a.overrideTime()
	}
	return time.Now()
}

type Option func(api *API)

func NewFiberAPIWithVersion(globalConfig *conf.GlobalConfiguration, opts ...Option) *API {
	api := &API{
		config:       globalConfig,
		version:      defaultVersion,
		overrideTime: time.Now,
	}

	for _, opt := range opts {
		opt(api)
	}
	if api.limiterOpts == nil {
		api.limiterOpts = NewLimiterOptions(globalConfig)
	}

	return api
}

func (a *API) SetupRoutes(app *fiber.App) {
	// Add global middleware
	app.Use(recover())
	app.Use(logger.New())

	// Configure CORS

	// Health check endpoint
	app.Get("/health", a.HealthCheck)

	logrus.WithField("version", a.version).Info("Routes configured for GoTrue API")
}
