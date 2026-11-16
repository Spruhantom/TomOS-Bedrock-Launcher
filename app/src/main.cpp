#include <QGuiApplication>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QProcess>
#include <QQuickStyle>
#include <QUrl>

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    QGuiApplication::setApplicationName(QStringLiteral("TomOS Bedrock Launcher"));
    QGuiApplication::setOrganizationName(QStringLiteral("TomOS"));
    QQuickStyle::setStyle(QStringLiteral("Basic"));

    QJsonObject dashboard;
    QProcess service;
    service.start(QStringLiteral(TOMOS_SERVICE_PATH));
    if (service.waitForStarted(2000) && service.waitForFinished(10000)
        && service.exitStatus() == QProcess::NormalExit && service.exitCode() == 0) {
        QJsonParseError parseError;
        const QJsonDocument document = QJsonDocument::fromJson(service.readAllStandardOutput(), &parseError);
        if (parseError.error == QJsonParseError::NoError && document.isObject()) {
            dashboard = document.object();
        }
    }

    const QJsonArray backends = dashboard.value(QStringLiteral("backends")).toArray();
    QJsonObject windows;
    QJsonObject android;
    for (const QJsonValue &value : backends) {
        const QJsonObject backend = value.toObject();
        const QString kind = backend.value(QStringLiteral("kind")).toString();
        if (kind == QStringLiteral("windows")) {
            windows = backend;
        } else if (kind == QStringLiteral("android")) {
            android = backend;
        }
    }

    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty(
        QStringLiteral("hostDistribution"),
        dashboard.value(QStringLiteral("distribution")).toString(QStringLiteral("Unknown Linux distribution")));
    engine.rootContext()->setContextProperty(
        QStringLiteral("hostArchitecture"),
        dashboard.value(QStringLiteral("architecture")).toString(QStringLiteral("Unknown")));
    engine.rootContext()->setContextProperty(
        QStringLiteral("hostSession"),
        dashboard.value(QStringLiteral("session_type")).toString(QStringLiteral("Unknown")));
    engine.rootContext()->setContextProperty(
        QStringLiteral("windowsDetected"),
        windows.value(QStringLiteral("available")).toBool(false));
    engine.rootContext()->setContextProperty(
        QStringLiteral("windowsStatus"),
        windows.value(QStringLiteral("status")).toString(QStringLiteral("Rust service unavailable")));
    engine.rootContext()->setContextProperty(
        QStringLiteral("androidDetected"),
        android.value(QStringLiteral("available")).toBool(false));
    engine.rootContext()->setContextProperty(
        QStringLiteral("androidStatus"),
        android.value(QStringLiteral("status")).toString(QStringLiteral("Rust service unavailable")));

    const QUrl mainUrl(QStringLiteral(
        "qrc:/qt/qml/TomOS/BedrockLauncher/qml/Main.qml"));
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(EXIT_FAILURE); },
        Qt::QueuedConnection);
    engine.load(mainUrl);
    return app.exec();
}
