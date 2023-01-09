package server

import (
	"encoding/json"
	"net/http"

	"master/pkg/config"
	"master/pkg/db"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/sirupsen/logrus"
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
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {})
	r.Route("/api", func(r chi.Router) {
		r.Use(middleware.Heartbeat("/"))
		r.Post("/update", nodeUpdate)
		r.Get("/status", status)
		r.Get("/status/{nodeID}", statusNode)
	})

	r.Group(func(r chi.Router) {
		r.Use(middleware.BasicAuth("testauth", map[string]string{
			"admin": "password123",
		}))

		r.Get("/admin", func(w http.ResponseWriter, r *http.Request) {
			w.Write([]byte("manage something here"))
		})
	})
	http.ListenAndServe(":8080", r)

	// err := http.ListenAndServeTLS(":8080", "cert.pem", "key.pem", r)
	// log.Fatal(err)
	// l := autocert.NewListener("")
	// err := http.ServeTLS(l, r, "cert.pem", "key.pem")
}

func nodeUpdate(w http.ResponseWriter, r *http.Request) {
	log.Printf("r body: %#v\n", r.Body)
	// var entries []db.WifiEntry // TODO decode into that
	test := make([]map[string]interface{}, 0)
	err := json.NewDecoder(r.Body).Decode(&test)
	log.Printf("test after decode: %v", test)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		log.Info("error:", err)
		return
	}
	w.Write([]byte("updated :) \n"))
}

func status(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte(`{"status": "ok"}`))
}

func statusNode(w http.ResponseWriter, r *http.Request) {
	s := chi.URLParam(r, "nodeID")
	log.Infof("Got request for node status: %s", s)
	w.Write([]byte(`{"node": "ok"}`))
}

func New(l *logrus.Logger, c *config.Config) *Server {
	log = l
	db.New(l, c)
	return &Server{}
}
