import javax.swing.JFrame;
import javax.swing.SwingUtilities;

import org.jfree.chart.ChartFactory;
import org.jfree.chart.ChartPanel;
import org.jfree.chart.JFreeChart;
import org.jfree.chart.plot.PlotOrientation;
import org.jfree.data.category.CategoryDataset;
import org.jfree.data.category.DefaultCategoryDataset;

import java.util.HashMap;
import java.util.Map;

public class BenchmarkClient {

    private static int noClients;
    private static int meanWaitTime;
    private static int noRequests;
    private static String [] queryNames;
    private static String [] workloadMix;
    private static Integer [][] workloadPerformance;
    private static int [][][] overallPerformance;

    private static void setWorkloadMix () {
        //Q1
        for (int i = 0; i < noRequests; i = i + 4) {
            queryNames[i] = "Query1";
            workloadMix[i] = "http://localhost:8000/sitzverteilung/2018";
        }

        //Q2
        for (int i = 1; i < noRequests; i = i + 10) {
            queryNames[i] = "Query2";
            workloadMix[i] = "http://localhost:8000/landtagsmitglieder/2018";
        }

        //Q3
        for (int i = 2; i < noRequests; i = i + 4) {
            queryNames[i] = "Query3";
            workloadMix[i] = "http://localhost:8000/sitzverteilung/2018";
        }

        //Q4
        for (int i = 3; i < noRequests; i = i + 10) {
            queryNames[i] = "Query4";
            workloadMix[i] = "http://localhost:8000/sitzverteilung/2018";
        }

        //Q5
        for (int i = 5; i < noRequests; i = i + 10) {
            queryNames[i] = "Query5";
            workloadMix[i] = "http://localhost:8000/knappstesieger/2018";
        }

        //Q6
        for (int i = 7; i < noRequests; i = i + 10) {
            queryNames[i] = "Query6";
            workloadMix[i] = "http://localhost:8000/knappstesieger/2018";
            queryNames[i + 2] = "Query6";
            workloadMix[i + 2] = "http://localhost:8000/knappstesieger/2018";
        }

    }

    private static int setWorkloadPointer () {
        double random = Math.random();
        random = random * noRequests;
        return ((int) random);
    }

    private static void handleThreads () {
        Terminal [] terminals = new Terminal [noClients];
        Thread [] threads = new Thread [noClients];
        for (int i = 0; i < noClients; i++) {
            int workloadPointer = setWorkloadPointer();
            Terminal t = new Terminal(meanWaitTime, noRequests, workloadMix, workloadPointer);
            terminals[i] = t;
            threads [i] = new Thread(t);
        }

        for (int i = 0; i < noClients; i++) {
            threads[i].start();
        }

        for (int i = 0; i < noClients; i++) {
            try {
                threads[i].join();
            }
            catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        for (int i = 0; i < noClients; i++) {
            Terminal terminal = terminals[i];
            workloadPerformance[i] = terminal.getWorkloadPerformance();
        }
    }

    private static int [] averagePerformances () {
        int [] averagePerformances = new int [6];

        int sum01 = 0;
        int counter01 = 0;
        int sum02 = 0;
        int counter02 = 0;
        int sum03 = 0;
        int counter03 = 0;
        int sum04 = 0;
        int counter04 = 0;
        int sum05 = 0;
        int counter05 = 0;
        int sum06 = 0;
        int counter06 = 0;

        for (int i = 0; i < noClients; i++) {
            for (int j = 0; j < noRequests; j = j + 4) {
                sum01 += workloadPerformance[i][j];
                counter01++;
            }
            for (int j = 1; j < noRequests; j = j + 10) {
                sum02 += workloadPerformance[i][j];
                counter02++;
            }
            for (int j = 2; j < noRequests; j = j + 4) {
                sum03 += workloadPerformance[i][j];
                counter03++;
            }
            for (int j = 3; j < noRequests; j = j + 10) {
                sum04 += workloadPerformance[i][j];
                counter04++;
            }
            for (int j = 5; j < noRequests; j = j + 10) {
                sum05 += workloadPerformance[i][j];
                counter05++;
            }
            for (int j = 7; j < noRequests; j = j + 10) {
                sum06 += workloadPerformance[i][j];
                counter06++;
                sum06 += workloadPerformance[i][j + 2];
                counter06++;
            }
        }
        averagePerformances[0] = sum01 / counter01;
        averagePerformances[1] = sum02 / counter02;
        averagePerformances[2] = sum03 / counter03;
        averagePerformances[3] = sum04 / counter04;
        averagePerformances[4] = sum05 / counter05;
        averagePerformances[5] = sum06 / counter06;

        return averagePerformances;
    }

    private static void setOverallPerformance (int n, int t) {
        int [] averagePerformances = averagePerformances();
        for (int i = 0; i < 6; i++) {
            overallPerformance[i][n][t] = averagePerformances[i];
        }
    }

    private static void generateTable () {

        TableGenerator tableGenerator = new TableGenerator(queryNames, workloadPerformance);
        tableGenerator.createTable();
    }

    private static void generateLineCharts () {
        String chartName;
        String xNameClients = "Anzahl Terminals n";
        String xNameTime = "Wartezeit t in s";
        String yName = "Dauer";

        int [] xValuesClients = new int [8];
        int [] xValuesTime = new int [8];
        xValuesClients[0] = 1;
        xValuesClients[1] = 2;
        xValuesClients[2] = 4;
        xValuesClients[3] = 8;
        xValuesClients[4] = 16;
        xValuesClients[5] = 32;
        xValuesClients[6] = 64;
        xValuesClients[7] = 128;
        xValuesTime[0] = 1;
        xValuesTime[1] = 2;
        xValuesTime[2] = 3;
        xValuesTime[3] = 4;
        xValuesTime[4] = 5;
        xValuesTime[5] = 6;
        xValuesTime[6] = 7;
        xValuesTime[7] = 8;

        for (int i = 0; i < 6; i ++) {
            chartName = "LineChart: Query" + (i + 1) + " bei konstantem n";
            LinechartGenerator linechartGenerator = new LinechartGenerator(xValuesTime, overallPerformance[i][0], xNameTime, yName, chartName);
            linechartGenerator.createChart();
        }

        for (int i = 0; i < 6; i ++) {
            chartName = "LineChart: Query" + (i + 1) + " bei konstantem t";
            int[] yValues = new int[8];
            for (int j = 0; j < 8; j++) {
                yValues[j] = overallPerformance[i][j][0];
            }
            LinechartGenerator linechartGenerator = new LinechartGenerator(xValuesClients, yValues, xNameClients, yName, chartName);
            linechartGenerator.createChart();
        }
    }



    public static void main (String [] args) {
        noRequests = 20;
        workloadMix = new String[noRequests];
        queryNames = new String[noRequests];

        setWorkloadMix();

        overallPerformance = new int [6][8][8];

        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                noClients = (int) Math.pow(2, i);
                meanWaitTime = 1000 * (j + 1);
                workloadPerformance = new Integer[noClients][noRequests];

                handleThreads();
                setOverallPerformance(i, j);
            }
        }

        generateTable();
        generateLineCharts();

    }
}
