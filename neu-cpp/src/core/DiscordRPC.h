#pragma once
#include <QObject>
#include <QString>
#include <QDebug>

// This is a bridge to the Discord RPC SDK
class DiscordRPC : public QObject {
    Q_OBJECT
public:
    explicit DiscordRPC(QObject *parent = nullptr) : QObject(parent) {
        // Initialize Discord RPC connection here
    }

    Q_INVOKABLE void updatePresence(const QString& title, const QString& artist, qint64 remainingTime) {
        qDebug() << "Updating Discord RPC: Listening to" << title << "by" << artist;
        // Call Discord_UpdatePresence(...)
    }

    Q_INVOKABLE void clearPresence() {
        qDebug() << "Clearing Discord RPC";
        // Call Discord_ClearPresence()
    }
};
