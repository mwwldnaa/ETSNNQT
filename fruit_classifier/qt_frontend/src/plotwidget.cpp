#include "plotwidget.h"
#include <QPainter>
#include <QPen>
#include <algorithm>

PlotWidget::PlotWidget(QWidget *parent) : QWidget(parent) {
    setBackgroundRole(QPalette::Base);
    setAutoFillBackground(true);
}

void PlotWidget::setData(const QVector<double> &data) {
    m_data = data;
    update();
}

void PlotWidget::paintEvent(QPaintEvent *) {
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    
    painter.fillRect(rect(), Qt::white);
    
    // Draw grid
    painter.setPen(QPen(Qt::lightGray, 1, Qt::DotLine));
    int gridSize = 5;
    for (int i = 0; i <= width(); i += width() / gridSize) {
        painter.drawLine(i, 0, i, height());
    }
    for (int i = 0; i <= height(); i += height() / gridSize) {
        painter.drawLine(0, i, width(), i);
    }

    if (m_data.isEmpty()) return;

    // Draw curve
    QPen curvePen(Qt::blue, 2);
    painter.setPen(curvePen);

    double maxVal = *std::max_element(m_data.begin(), m_data.end());
    double minVal = *std::min_element(m_data.begin(), m_data.end());
    if (maxVal == minVal) maxVal += 1.0;

    QPointF prevPoint;
    for (int i = 0; i < m_data.size(); ++i) {
        double x = i * width() / double(m_data.size() - 1);
        double y = height() - ((m_data[i] - minVal) / (maxVal - minVal)) * height();

        if (i > 0) {
            painter.drawLine(prevPoint, QPointF(x, y));
        }
        prevPoint = QPointF(x, y);
    }

    // Draw axis labels
    painter.setPen(Qt::black);
    painter.drawText(QRect(0, height() - 20, width(), 20), 
                    Qt::AlignLeft | Qt::AlignVCenter, "0");
    painter.drawText(QRect(0, height() - 20, width() - 5, 20), 
                    Qt::AlignRight | Qt::AlignVCenter, QString::number(m_data.size() - 1));
    painter.drawText(QRect(0, 0, 50, 20), 
                    Qt::AlignLeft | Qt::AlignTop, QString::number(maxVal, 'f', 2));
    painter.drawText(QRect(0, height() - 20, 50, 20), 
                    Qt::AlignLeft | Qt::AlignTop, QString::number(minVal, 'f', 2));
}