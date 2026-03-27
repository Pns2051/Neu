#pragma once
#include <QObject>
#include <QtGlobal>
#include <QMediaPlayer>
#include <QAudioOutput>
#include <QUrl>
#include <QString>

class PlaybackEngine : public QObject {
    Q_OBJECT
public:
    explicit PlaybackEngine(QObject *parent = nullptr) : QObject(parent) {
        player = new QMediaPlayer(this);
        audioOutput = new QAudioOutput(this);
        player->setAudioOutput(audioOutput);
        
        connect(player, &QMediaPlayer::positionChanged, this, &PlaybackEngine::positionChanged);
        connect(player, &QMediaPlayer::durationChanged, this, &PlaybackEngine::durationChanged);
        connect(player, &QMediaPlayer::playbackStateChanged, this, [this]() {
            emit playingChanged(player->playbackState() == QMediaPlayer::PlayingState);
        });
    }

    void play(const QString& url) {
        player->setSource(QUrl(url));
        player->play();
    }

    void toggle() {
        if (player->playbackState() == QMediaPlayer::PlayingState) {
            player->pause();
        } else {
            player->play();
        }
    }

    void setVolume(float volume) {
        audioOutput->setVolume(volume);
    }

    void seek(qint64 position) {
        player->setPosition(position);
    }

signals:
    void positionChanged(qint64 position);
    void durationChanged(qint64 duration);
    void playingChanged(bool isPlaying);

private:
    QMediaPlayer* player;
    QAudioOutput* audioOutput;
};
