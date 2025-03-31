package api

import "github.com/gofiber/fiber/v3"

type HealthCheckResponse struct {
	Version     string `json:"version"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

func (a *API) HealthCheck(ctx fiber.Ctx) error {
	healthCheckResponse := HealthCheckResponse{
		Version:     a.Version(),
		Name:        "rcauth",
		Description: "Red Cardinal Authentication Service",
	}

	return ctx.JSON(healthCheckResponse)
}
