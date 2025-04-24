/********************************************************************************
** Form generated from reading UI file 'mainwindow.ui'
**
** Created by: Qt User Interface Compiler version 6.4.2
**
** WARNING! All changes made in this file will be lost when recompiling UI file!
********************************************************************************/

#ifndef UI_MAINWINDOW_H
#define UI_MAINWINDOW_H

#include <QtCore/QVariant>
#include <QtWidgets/QApplication>
#include <QtWidgets/QLabel>
#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QWidget>

QT_BEGIN_NAMESPACE

class Ui_MainWindow
{
public:
    QWidget *centralWidget;
    QVBoxLayout *verticalLayout;
    QPushButton *trainButton;
    QLabel *finalAccuracyLabel;
    QLabel *epochsLabel;
    QLabel *label;
    QWidget *accuracyChartView;
    QLabel *label_2;
    QWidget *lossChartView;

    void setupUi(QMainWindow *MainWindow)
    {
        if (MainWindow->objectName().isEmpty())
            MainWindow->setObjectName("MainWindow");
        MainWindow->resize(800, 800);
        centralWidget = new QWidget(MainWindow);
        centralWidget->setObjectName("centralWidget");
        verticalLayout = new QVBoxLayout(centralWidget);
        verticalLayout->setObjectName("verticalLayout");
        trainButton = new QPushButton(centralWidget);
        trainButton->setObjectName("trainButton");
        trainButton->setMinimumSize(QSize(0, 40));

        verticalLayout->addWidget(trainButton);

        finalAccuracyLabel = new QLabel(centralWidget);
        finalAccuracyLabel->setObjectName("finalAccuracyLabel");
        finalAccuracyLabel->setAlignment(Qt::AlignCenter);
        QFont font;
        font.setPointSize(12);
        font.setBold(true);
        finalAccuracyLabel->setFont(font);

        verticalLayout->addWidget(finalAccuracyLabel);

        epochsLabel = new QLabel(centralWidget);
        epochsLabel->setObjectName("epochsLabel");
        epochsLabel->setAlignment(Qt::AlignCenter);

        verticalLayout->addWidget(epochsLabel);

        label = new QLabel(centralWidget);
        label->setObjectName("label");
        label->setAlignment(Qt::AlignCenter);

        verticalLayout->addWidget(label);

        accuracyChartView = new QWidget(centralWidget);
        accuracyChartView->setObjectName("accuracyChartView");
        accuracyChartView->setMinimumSize(QSize(600, 300));

        verticalLayout->addWidget(accuracyChartView);

        label_2 = new QLabel(centralWidget);
        label_2->setObjectName("label_2");
        label_2->setAlignment(Qt::AlignCenter);

        verticalLayout->addWidget(label_2);

        lossChartView = new QWidget(centralWidget);
        lossChartView->setObjectName("lossChartView");
        lossChartView->setMinimumSize(QSize(600, 300));

        verticalLayout->addWidget(lossChartView);

        MainWindow->setCentralWidget(centralWidget);

        retranslateUi(MainWindow);

        QMetaObject::connectSlotsByName(MainWindow);
    } // setupUi

    void retranslateUi(QMainWindow *MainWindow)
    {
        MainWindow->setWindowTitle(QCoreApplication::translate("MainWindow", "Fruit Classifier", nullptr));
        trainButton->setText(QCoreApplication::translate("MainWindow", "Train Network", nullptr));
        finalAccuracyLabel->setText(QCoreApplication::translate("MainWindow", "Final Accuracy: N/A", nullptr));
        epochsLabel->setText(QCoreApplication::translate("MainWindow", "Epochs: 0", nullptr));
        label->setText(QCoreApplication::translate("MainWindow", "Accuracy Progress", nullptr));
        label_2->setText(QCoreApplication::translate("MainWindow", "Loss Progress", nullptr));
    } // retranslateUi

};

namespace Ui {
    class MainWindow: public Ui_MainWindow {};
} // namespace Ui

QT_END_NAMESPACE

#endif // UI_MAINWINDOW_H
