package main

// TODO
// API server receiving scan results as input from multiple sources
//		Stores them in DB (Gorm/postgres backend)
//		authenticates the messenger (by signature? universal node privkey, preshared)
//		SSL (pre-shared master key)
// https://github.com/Sirupsen/logrus logger

import (
	"os"
	"time"

	"github.com/sirupsen/logrus"

	"master/pkg/config"
	"master/pkg/server"
)

const (
	CONFIG_DIR = "./config/"
)

var (
	log    = logrus.New()
	CONFIG config.Config
)

func init() {
	// logfile, err := os.OpenFile("main.log", os.O_WRONLY|os.O_CREATE, 0755)
	// if err != nil {
	// 	log.Fatal("Failed to open logfile, %v", err)
	// }
	// mw := io.MultiWriter(os.Stdout, logfile)
	// log.SetOutput(mw)
	log.Out = os.Stdout
	log.SetLevel(logrus.DebugLevel)
	log.SetFormatter(&logrus.TextFormatter{})

	cfg, err := config.LoadConfig(CONFIG_DIR)
	if err != nil {
		log.Fatal("Failed to load config, %v", err)
	}
	CONFIG = cfg
}

func main() {
	log.Infof("Started master, t: %s\n", time.Now().String())
	s := server.New(log, &CONFIG)
	s.Test()

	log.WithFields(logrus.Fields{
		"animal": "walrus",
	}).Info("A walrus appears")

	s.Serve()

}
