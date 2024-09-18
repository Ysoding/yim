package ipconfig

import (
	"context"

	"github.com/Ysoding/yim/internal/ipconfig/domain"
	"github.com/Ysoding/yim/internal/ipconfig/source"
	"github.com/cloudwego/hertz/pkg/app"
	"github.com/cloudwego/hertz/pkg/app/server"
	"github.com/cloudwego/hertz/pkg/protocol/consts"
)

func Run() {
	source.Init()
	domain.Init()
	source.StartMock()

	h := server.Default(server.WithHostPorts(":6789"))

	h.GET("/ips", GetIPs)

	h.GET("/healthy", func(ctx context.Context, c *app.RequestContext) {
		c.JSON(consts.StatusOK, "ok")
	})

	h.Spin()
}
