#pragma once
#include <QObject>
#include <QNetworkAccessManager>
#include <QNetworkReply>
#include <QJsonDocument>
#include <QJsonObject>
#include <QSettings>
#include <QDebug>

class UpdateChecker : public QObject {
    Q_OBJECT
public:
    explicit UpdateChecker(QObject *parent = nullptr) : QObject(parent) {
        settings = new QSettings("Pns2051", "Neu", this);
    }

    Q_INVOKABLE void checkForUpdates() {
        if (!settings->value("updates/enabled", true).toBool()) {
            qDebug() << "Update checker disabled by user.";
            return;
        }

        auto* reply = manager.get(QNetworkRequest(QUrl("https://api.github.com/repos/Pns2051/neu/releases/latest")));
        connect(reply, &QNetworkReply::finished, this, [this, reply]() {
            if (reply->error() == QNetworkReply::NoError) {
                QJsonDocument doc = QJsonDocument::fromJson(reply->readAll());
                QString latestVersion = doc.object()["tag_name"].toString();
                if (latestVersion != "1.0.0") { // Hardcoded current version for demo
                    emit updateAvailable(latestVersion);
                }
            }
            reply->deleteLater();
        });
    }

    Q_INVOKABLE bool isUpdatesEnabled() const { return settings->value("updates/enabled", true).toBool(); }
    Q_INVOKABLE void setUpdatesEnabled(bool enabled) { settings->setValue("updates/enabled", enabled); }

signals:
    void updateAvailable(const QString& version);

private:
    QNetworkAccessManager manager;
    QSettings* settings;
};
