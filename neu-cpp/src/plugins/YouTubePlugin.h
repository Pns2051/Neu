#pragma once
#include "../core/MusicPlugin.h"

class YouTubePlugin : public MusicPlugin {
    Q_OBJECT
public:
    QString name() const override;
    PluginType type() const override { return PluginType::Search; }
    QString author() const override { return "Neu Core"; }
    
    QVector<UnifiedTrack> search(const QString& query) override;
    QString stream(const QString& trackId) override;
};
