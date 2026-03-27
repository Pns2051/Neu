#pragma once
#include <QObject>
#include <QVector>
#include <QMap>
#include <memory>
#include "MusicPlugin.h"

class PluginRegistry : public QObject {
    Q_OBJECT
public:
    static PluginRegistry& instance() {
        static PluginRegistry inst;
        return inst;
    }

    void registerPlugin(MusicPlugin* plugin) {
        if (!plugin) return;
        plugins.append(plugin);
        emit pluginRegistered(plugin->name());
    }

    QVector<MusicPlugin*> getPlugins() const {
        return plugins;
    }

    MusicPlugin* getPlugin(const QString& name) const {
        for (auto* p : plugins) {
            if (p->name() == name) return p;
        }
        return nullptr;
    }

signals:
    void pluginRegistered(const QString& name);

private:
    PluginRegistry() = default;
    QVector<MusicPlugin*> plugins;
};
