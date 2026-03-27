#pragma once
#include <QObject>
#include <QOAuth2AuthorizationCodeFlow>
#include <QOAuthHttpServerReplyHandler>
#include <QDesktopServices>
#include <QUrl>
#include <QSettings>

class AuthManager : public QObject {
    Q_OBJECT
public:
    explicit AuthManager(QObject *parent = nullptr) : QObject(parent) {
        setupYouTube();
        setupSpotify();
    }

    Q_INVOKABLE void loginYouTube() {
        youtubeOAuth.grant();
    }

    Q_INVOKABLE void loginSpotify() {
        spotifyOAuth.grant();
    }

signals:
    void authenticated(const QString& service, bool success);

private:
    void setupYouTube() {
        youtubeOAuth.setAuthorizationUrl(QUrl("https://accounts.google.com/o/oauth2/auth"));
        youtubeOAuth.setAccessTokenUrl(QUrl("https://oauth2.googleapis.com/token"));
        youtubeOAuth.setClientIdentifier("neu_yt_client_id");
        youtubeOAuth.setClientIdentifierSharedKey("neu_yt_client_secret");
        youtubeOAuth.setScope("https://www.googleapis.com/auth/youtube.readonly");

        auto *replyHandler = new QOAuthHttpServerReplyHandler(1234, this);
        youtubeOAuth.setReplyHandler(replyHandler);

        connect(&youtubeOAuth, &QOAuth2AuthorizationCodeFlow::authorizeWithBrowser, [this](const QUrl &url) {
            QDesktopServices::openUrl(url);
        });

        connect(&youtubeOAuth, &QOAuth2AuthorizationCodeFlow::granted, [this]() {
            emit authenticated("YouTube", true);
        });
    }

    void setupSpotify() {
        spotifyOAuth.setAuthorizationUrl(QUrl("https://accounts.spotify.com/authorize"));
        spotifyOAuth.setAccessTokenUrl(QUrl("https://accounts.spotify.com/api/token"));
        spotifyOAuth.setClientIdentifier("neu_spotify_client_id");
        spotifyOAuth.setClientIdentifierSharedKey("neu_spotify_client_secret");
        spotifyOAuth.setScope("user-library-read playlist-read-private");

        auto *replyHandler = new QOAuthHttpServerReplyHandler(1235, this);
        spotifyOAuth.setReplyHandler(replyHandler);

        connect(&spotifyOAuth, &QOAuth2AuthorizationCodeFlow::authorizeWithBrowser, [this](const QUrl &url) {
            QDesktopServices::openUrl(url);
        });

        connect(&spotifyOAuth, &QOAuth2AuthorizationCodeFlow::granted, [this]() {
            emit authenticated("Spotify", true);
        });
    }

    QOAuth2AuthorizationCodeFlow youtubeOAuth;
    QOAuth2AuthorizationCodeFlow spotifyOAuth;
};
