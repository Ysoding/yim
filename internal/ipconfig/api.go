package ipconfig

import (
	"context"

	"github.com/Ysoding/yim/internal/ipconfig/domain"
	"github.com/cloudwego/hertz/pkg/app"
	"github.com/cloudwego/hertz/pkg/common/utils"
	"github.com/cloudwego/hertz/pkg/protocol/consts"
)

type Response struct {
	Message string      `json:"message"`
	Code    int         `json:"code"`
	Data    interface{} `json:"data"`
}

func GetIPs(ctx context.Context, appCtx *app.RequestContext) {
	defer func() {
		if err := recover(); err != nil {
			appCtx.JSON(consts.StatusBadRequest, utils.H{"err": err})
		}
	}()

	ipCtx := domain.NewIPConfigContext(ctx, appCtx)

	eds := domain.Dispatch(ipCtx)
	appCtx.JSON(consts.StatusOK, Response{
		Message: "ok",
		Code:    0,
		Data:    top5Endports(eds),
	})

}

func top5Endports(eds []*domain.Endpoint) []*domain.Endpoint {
	if len(eds) < 5 {
		return eds
	}
	return eds[:5]
}
