package server

import (
	"net/http"

	"master/pkg/config"
	"master/pkg/db"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/sirupsen/logrus"
	"github.com/kr/pretty"
)

var log *logrus.Logger

type Server struct{}

func (s Server) Test() {
	log.Printf("Server Test\n")
	db.GetSomeShit()
}

func (s Server) Serve() {
	log.Info("Started server.Serve()")
	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Hello world\n"))
	})
	r.Post("/update", nodeUpdates)
	r.Get("/status", status)
	r.Get("/status/{nodeID}", statusNode)
	http.ListenAndServe(":8080", r)
}

func nodeUpdates(w http.ResponseWriter, r *http.Request) {
	log.Infof("%#v", r)
	w.Write([]byte("updated :) \n"))
}

func status(w http.ResponseWriter, r *http.Request) {
	log.Infof("%#v", r)
	w.Write([]byte("status \n"))
}

func statusNode(w http.ResponseWriter, r *http.Request) {
	s := chi.URLParam(r, "nodeID")
	log.Infof("Got request for node status: %s", s)
	log.Infof("%# v", pretty.Formatter(r))
	w.Write([]byte("status for node " + s + "\n"))
}

func New(l *logrus.Logger, c *config.Config) *Server {
	log = l
	db.New(l, c)
	return &Server{}
}
