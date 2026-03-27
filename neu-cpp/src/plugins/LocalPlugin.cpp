#include "LocalPlugin.h"
#include <QDirIterator>
#include <QFileInfo>

QString LocalPlugin::name() const {
    return "Local Files";
}

QVector<UnifiedTrack> LocalPlugin::search(const QString& query) {
    QVector<UnifiedTrack> results;
    QString musicPath = QDir::homePath() + "/Music";
    QDirIterator it(musicPath, {"*.mp3", "*.flac", "*.wav"}, QDir::Files, QDirIterator::Subdirectories);
    
    while (it.hasNext()) {
        it.next();
        QFileInfo info = it.fileInfo();
        if (info.fileName().contains(query, Qt::CaseInsensitive)) {
            UnifiedTrack track;
            track.id = info.absoluteFilePath();
            track.title = info.baseName();
            track.artist = "Local Artist";
            track.source = "local";
            results.push_back(track);
        }
    }
    return results;
}

QString LocalPlugin::stream(const QString& trackId) {
    return "file://" + trackId;
}
