package server

import (
	"master/pkg/config"
	"master/pkg/db"

	"github.com/sirupsen/logrus"
)

var log *logrus.Logger

type Server struct{}

func (s Server) Test() {
	log.Printf("Server Test\n")
	db.GetSomeShit()
}

func (s Server) Serve() {
	log.Info("Message from server.Serve()")
}

func New(l *logrus.Logger, c *config.Config) *Server {
	log = l
	db.New(l, c)
	return &Server{}
}
