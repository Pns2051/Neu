#pragma once
#include <QObject>
#include <QNetworkAccessManager>
#include <QNetworkReply>
#include <QJsonDocument>
#include <QJsonArray>
#include <QJsonObject>
#include <QUrlQuery>

class LyricsProvider : public QObject {
    Q_OBJECT
public:
    explicit LyricsProvider(QObject *parent = nullptr) : QObject(parent) {}

    void fetchLyrics(const QString& artist, const QString& title) {
        QUrl url("https://lrclib.net/api/get");
        QUrlQuery query;
        query.addQueryItem("artist_name", artist);
        query.addQueryItem("track_name", title);
        url.setQuery(query);

        auto *reply = manager.get(QNetworkRequest(url));
        connect(reply, &QNetworkReply::finished, this, [this, reply]() {
            if (reply->error() == QNetworkReply::NoError) {
                QJsonDocument doc = QJsonDocument::fromJson(reply->readAll());
                QString lyrics = doc.object()["syncedLyrics"].toString();
                if (lyrics.isEmpty()) lyrics = doc.object()["plainLyrics"].toString();
                emit lyricsReady(lyrics);
            }
            reply->deleteLater();
        });
    }

signals:
    void lyricsReady(const QString& lyrics);

private:
    QNetworkAccessManager manager;
};
