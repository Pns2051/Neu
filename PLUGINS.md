# Neu Music Platform - Extension Development Guide

Neu is designed to be highly extensible. This guide explains how you can create your own music source extensions (plugins) and integrate them into the platform.

## Getting Started

All Neu extensions are written in C++ using the Qt framework. To create an extension, you need to implement the `MusicPlugin` interface.

### 1. Implement the `MusicPlugin` Interface

Create a new class that inherits from `MusicPlugin`. You must override the following methods:

```cpp
#include "core/MusicPlugin.h"

class MyAwesomePlugin : public MusicPlugin {
    Q_OBJECT
public:
    QString name() const override { return "My Awesome Source"; }
    PluginType type() const override { return PluginType::Search; }
    QString author() const override { return "Developer Name"; }
    
    QVector<UnifiedTrack> search(const QString& query) override {
        // Implement your search logic here
        QVector<UnifiedTrack> results;
        // ...
        return results;
    }

    QString stream(const QString& trackId) override {
        // Return the playback URL for the given track ID
        return "https://example.com/stream/" + trackId;
    }
};
```

### 2. Register Your Plugin

To make Neu aware of your plugin, you need to register it with the `PluginRegistry` in `main.cpp`:

```cpp
#include "core/PluginRegistry.h"
#include "plugins/MyAwesomePlugin.h"

// inside main()
PluginRegistry::instance().registerPlugin(new MyAwesomePlugin());
```

### 3. Build and Run

Add your source files to `CMakeLists.txt` and rebuild the project:

```cmake
add_executable(neu-cpp
    # ...
    src/plugins/MyAwesomePlugin.cpp
)
```

## Plugin Types

Currently, Neu supports the following plugin types:
- `Search`: Provides track search results.
- `Stream`: Provides playback URLs.
- `Lyrics`: Provides track lyrics.
- `Social`: Integrates with social services (Spotify/YouTube OAuth).

## Best Practices
- **Asynchronous Operations**: If your search involves network requests, use `QNetworkAccessManager` asynchronously.
- **Resource Management**: Neu manages the lifecycle of registered plugins, but ensure you handle your own internal resources correctly.
- **UI Integration**: Plugins are automatically discovered and can be exposed to QML via the `plugins` context property.
