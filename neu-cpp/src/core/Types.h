#pragma once
#include <QString>
#include <QMetaType>
#include <QObject>

struct UnifiedTrack {
    Q_GADGET
    Q_PROPERTY(QString id MEMBER id)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QString artist MEMBER artist)
    Q_PROPERTY(unsigned int duration MEMBER duration)
    Q_PROPERTY(QString source MEMBER source)

public:
    QString id;
    QString title;
    QString artist;
    unsigned int duration;
    QString source; // "local" or "youtube"
};

Q_DECLARE_METATYPE(UnifiedTrack)
