package db

import (
	"gorm.io/driver/postgres"
	"gorm.io/gorm"

	"fmt"

	"master/pkg/config"

	"github.com/sirupsen/logrus"
)

var (
	log *logrus.Logger
	DB  *gorm.DB
)

type WifiEntry struct {
	gorm.Model
	Lat  float32
	Lon  float32
	Ssid string
}

func connectDB(config *config.Config) {
	var err error
	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s sslmode=disable TimeZone=Asia/Shanghai", config.DBHost, config.DBUserName, config.DBUserPassword, config.DBName)
	log.Debugf("Opening database, config: %s", dsn)

	DB, err = gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatal("Failed to connect to the Database")
	}
	log.Info("Connected to the database")
}

func New(l *logrus.Logger, c *config.Config) {
	log = l
	connectDB(c)
	DB.AutoMigrate(&WifiEntry{})
	log.Info("initialized db")
	DB.Create(&WifiEntry{Lat: 21.37, Lon: 14.88, Ssid: "TestSSID1"})
}

func GetSomeShit() {
	var e WifiEntry
	DB.First(&e, "")
	log.Infof("e is %#v", e)
	DB.Model(&e).Update("lat", 12.34)
	log.Infof("e is %#v", e)
	DB.Model(&e).Updates(map[string]interface{}{"lon": 11.11, "ssid": "changed"})
	log.Infof("e is %#v", e)
	DB.Delete(&e, 1)

}
