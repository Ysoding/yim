package ipconfig

import (
	"context"

	"github.com/cloudwego/hertz/pkg/app"
)

type Response struct {
	Message string      `json:"message"`
	Code    int         `json:"code"`
	Data    interface{} `json:"data"`
}

func GetIPs(c context.Context, ctx *app.RequestContext) {

}
