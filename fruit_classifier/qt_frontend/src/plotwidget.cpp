#include "plotwidget.h"
#include <QPainter>
#include <QPen>
#include <algorithm>

PlotWidget::PlotWidget(QWidget *parent) : QWidget(parent), 
    m_plotColor(Qt::blue), 
    m_minY(0.0), 
    m_maxY(1.0)
{
    setBackgroundRole(QPalette::Base);
    setAutoFillBackground(true);
}

void PlotWidget::setData(const QVector<double> &data) {
    m_data = data;
    update();
}

void PlotWidget::setPlotColor(const QColor &color) {
    m_plotColor = color;
    update();
}

void PlotWidget::setYRange(double minY, double maxY) {
    m_minY = minY;
    m_maxY = maxY;
    update();
}

void PlotWidget::setTitle(const QString &title) {
    m_title = title;
    update();
}

void PlotWidget::paintEvent(QPaintEvent *) {
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    
    painter.fillRect(rect(), Qt::white);
    
    if (!m_title.isEmpty()) {
        painter.setPen(Qt::black);
        QFont font = painter.font();
        font.setBold(true);
        painter.setFont(font);
        painter.drawText(10, 20, m_title);
    }
    
    painter.setPen(QPen(Qt::lightGray, 1, Qt::DotLine));
    int gridSize = 5;
    for (int i = 0; i <= width(); i += width() / gridSize) {
        painter.drawLine(i, 0, i, height());
    }
    for (int i = 0; i <= height(); i += height() / gridSize) {
        painter.drawLine(0, i, width(), i);
    }

    if (m_data.isEmpty()) return;

    QPen curvePen(m_plotColor, 2);
    painter.setPen(curvePen);

    QPointF prevPoint;
    for (int i = 0; i < m_data.size(); ++i) {
        double x = i * width() / double(m_data.size() - 1);
        double y = height() - ((m_data[i] - m_minY) / (m_maxY - m_minY)) * height();
        y = qMax(0.0, qMin(double(height()), y));

        if (i > 0) {
            painter.drawLine(prevPoint, QPointF(x, y));
        }
        prevPoint = QPointF(x, y);
    }

    painter.setPen(Qt::black);
    QFont font = painter.font();
    font.setPointSize(8);
    painter.setFont(font);

    painter.drawText(QRect(0, height() - 20, 30, 20), 
                   Qt::AlignLeft | Qt::AlignVCenter, "0");
    painter.drawText(QRect(width() - 30, height() - 20, 30, 20), 
                   Qt::AlignRight | Qt::AlignVCenter, QString::number(m_data.size() - 1));
    painter.drawText(QRect(5, 0, 50, 20), 
                   Qt::AlignLeft | Qt::AlignTop, QString::number(m_maxY, 'f', 2));
    painter.drawText(QRect(5, height() - 20, 50, 20), 
                   Qt::AlignLeft | Qt::AlignTop, QString::number(m_minY, 'f', 2));
}