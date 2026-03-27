#pragma once
#include <QObject>
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QSqlError>
#include <QVector>
#include <QDebug>
#include "MusicPlugin.h"

class PlaylistManager : public QObject {
    Q_OBJECT
public:
    explicit PlaylistManager(QObject *parent = nullptr) : QObject(parent) {
        db = QSqlDatabase::addDatabase("QSQLITE");
        db.setDatabaseName("neu_playlists.db");
        if (!db.open()) {
            qDebug() << "Error: connection with database failed";
        } else {
            QSqlQuery query;
            query.exec("CREATE TABLE IF NOT EXISTS playlists (id INTEGER PRIMARY KEY, name TEXT)");
            query.exec("CREATE TABLE IF NOT EXISTS tracks (id INTEGER PRIMARY KEY, playlist_id INTEGER, track_id TEXT, title TEXT, artist TEXT)");
        }
    }

#include "Types.h"

    Q_INVOKABLE void createPlaylist(const QString& name) {
        QSqlQuery query;
        query.prepare("INSERT INTO playlists (name) VALUES (?)");
        query.addBindValue(name);
        query.exec();
        emit playlistsChanged();
    }

    Q_INVOKABLE void addTrackToPlaylist(int playlistId, const UnifiedTrack& track) {
        QSqlQuery query;
        query.prepare("INSERT INTO tracks (playlist_id, track_id, title, artist) VALUES (?, ?, ?, ?)");
        query.addBindValue(playlistId);
        query.addBindValue(track.id);
        query.addBindValue(track.title);
        query.addBindValue(track.artist);
        query.exec();
    }

signals:
    void playlistsChanged();

private:
    QSqlDatabase db;
};
