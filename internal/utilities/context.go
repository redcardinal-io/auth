package utilities

import (
	"context"
	"sync"
)

// WaitForCleanup waits until all long-running goroutines shut
// down cleanly or until the provided context signals done.
func WaitForCleanup(ctx context.Context, wg *sync.WaitGroup) {
	cleanupDone := make(chan struct{})

	go func() {
		defer close(cleanupDone)

		wg.Wait()
	}()

	select {
	case <-ctx.Done():
		return

	case <-cleanupDone:
		return
	}
}
