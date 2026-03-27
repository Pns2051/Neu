#pragma once
#include <QString>
#include <QMetaType>

struct UnifiedTrack {
    QString id;
    QString title;
    QString artist;
    unsigned int duration;
    QString source; // "local" or "youtube"
};

Q_DECLARE_METATYPE(UnifiedTrack)
