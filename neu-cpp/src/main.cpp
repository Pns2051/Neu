#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QUrl>
#include <QString>

#include "core/AuthManager.h"
#include "core/LyricsProvider.h"
#include "core/DiscordRPC.h"
#include "core/PlaylistManager.h"
#include "core/PlaybackEngine.h"
#include "core/UpdateChecker.h"
#include "core/PluginRegistry.h"
#include "plugins/YouTubePlugin.h"
#include "plugins/LocalPlugin.h"

int main(int argc, char *argv[]) {
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;

    PlaybackEngine playback;
    UpdateChecker updater;
    AuthManager auth;
    LyricsProvider lyrics;
    DiscordRPC discord;
    PlaylistManager playlists;

    // Plugin System
    auto& registry = PluginRegistry::instance();
    registry.registerPlugin(new YouTubePlugin());
    registry.registerPlugin(new LocalPlugin());

    engine.rootContext()->setContextProperty("playback", &playback);
    engine.rootContext()->setContextProperty("updater", &updater);
    engine.rootContext()->setContextProperty("auth", &auth);
    engine.rootContext()->setContextProperty("lyricsProvider", &lyrics);
    engine.rootContext()->setContextProperty("discord", &discord);
    engine.rootContext()->setContextProperty("playlists", &playlists);
    engine.rootContext()->setContextProperty("plugins", &registry);

    const QUrl url(u"qrc:/ui/main.qml"_qs);
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);
    engine.load(url);

    updater.checkForUpdates();

    return app.exec();
}
