#include <QObject>
#include <QString>
#include <QVector>
#include <QtGlobal>
#include "Types.h"

enum class PluginType {
    Search,
    Stream,
    Social,
    Lyrics
};

class MusicPlugin : public QObject {
    Q_OBJECT
public:
    virtual QString name() const = 0;
    virtual QString author() const { return "Neu Team"; }
    virtual QString version() const { return "1.0.0"; }
    virtual QString description() const { return "A Neu Music Platform Extension"; }
    virtual PluginType type() const = 0;

    virtual QVector<UnifiedTrack> search(const QString& query) = 0;
    virtual QString stream(const QString& trackId) = 0;
};
