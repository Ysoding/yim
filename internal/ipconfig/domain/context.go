package domain

import (
	"context"

	"github.com/cloudwego/hertz/pkg/app"
)

type IPConfigContext struct {
	Ctx       context.Context
	AppCtx    *app.RequestContext
	ClientCtx *ClientContext
}

type ClientContext struct {
	IP string `json:"ip"`
}

func NewIPConfigContext(ctx context.Context, appCtx *app.RequestContext) *IPConfigContext {
	return &IPConfigContext{
		Ctx:    ctx,
		AppCtx: appCtx,
	}
}
