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

        //Q3.1
        for (int i = 2; i < noRequests; i = i + 16) {
            queryNames[i] = "Query3.1";
            workloadMix[i] = "http://localhost:8000/wahlbeteiligung/1/2018";
        }

        //Q3.2
        for (int i = 6; i < noRequests; i = i + 16) {
            queryNames[i] = "Query3.2";
            workloadMix[i] = "http://localhost:8000/wahlbeteiligung/1/2018";
        }

        //Q3.3
        for (int i = 10; i < noRequests; i = i + 16) {
            queryNames[i] = "Query3.3";
            workloadMix[i] = "http://localhost:8000/stimmverteilung/122/2018";
        }

        //Q3.4
        for (int i = 14; i < noRequests; i = i + 16) {
            queryNames[i] = "Query3.4";
            workloadMix[i] = "http://localhost:8000/stimmverteilungdifferenz/122";
        }

        //Q4.1
        for (int i = 3; i < noRequests; i = i + 20) {
            queryNames[i] = "Query4.1";
            workloadMix[i] = "http://localhost:8000/siegerpartei/erststimmen/122/2018";
        }

        //Q4.2
        for (int i = 13; i < noRequests; i = i + 20) {
            queryNames[i] = "Query4.2";
            workloadMix[i] = "http://localhost:8000/siegerpartei/zweitstimmen/122/2018";
        }

        //Q5
        for (int i = 5; i < noRequests; i = i + 10) {
            queryNames[i] = "Query5";
            workloadMix[i] = "http://localhost:8000/ueberhangmandate/7/1301/2018";
        }

        //Q6.1
        for (int i = 7; i < noRequests; i = i + 10) {
            queryNames[i] = "Query6.1";
            workloadMix[i] = "http://localhost:8000/knappstesieger/1301/2018";
        }

        //Q6.2
        for (int i = 9; i < noRequests; i = i + 10) {
            queryNames[i] = "Query6.2";
            workloadMix[i] = "http://localhost:8000/knappstesieger/1/2018";
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
        int [] averagePerformances = new int [11];

        int sum01 = 0;
        int counter01 = 0;
        int sum02 = 0;
        int counter02 = 0;
        int sum031 = 0;
        int counter031 = 0;
        int sum032 = 0;
        int counter032 = 0;
        int sum033 = 0;
        int counter033 = 0;
        int sum034 = 0;
        int counter034 = 0;
        int sum041 = 0;
        int counter041 = 0;
        int sum042 = 0;
        int counter042 = 0;
        int sum05 = 0;
        int counter05 = 0;
        int sum061 = 0;
        int counter061 = 0;
        int sum062 = 0;
        int counter062 = 0;

        for (int i = 0; i < noClients; i++) {
            for (int j = 0; j < noRequests; j = j + 4) {
                sum01 += workloadPerformance[i][j];
                counter01++;
            }
            for (int j = 1; j < noRequests; j = j + 10) {
                sum02 += workloadPerformance[i][j];
                counter02++;
            }
            for (int j = 2; j < noRequests; j = j + 16) {
                sum031 += workloadPerformance[i][j];
                counter031++;
            }
            for (int j = 6; j < noRequests; j = j + 16) {
                sum032 += workloadPerformance[i][j];
                counter032++;
            }
            for (int j = 10; j < noRequests; j = j + 16) {
                sum033 += workloadPerformance[i][j];
                counter033++;
            }
            for (int j = 14; j < noRequests; j = j + 16) {
                sum034 += workloadPerformance[i][j];
                counter034++;
            }
            for (int j = 3; j < noRequests; j = j + 20) {
                sum041 += workloadPerformance[i][j];
                counter041++;
            }
            for (int j = 13; j < noRequests; j = j + 20) {
                sum042 += workloadPerformance[i][j];
                counter042++;
            }
            for (int j = 5; j < noRequests; j = j + 10) {
                sum05 += workloadPerformance[i][j];
                counter05++;
            }
            for (int j = 7; j < noRequests; j = j + 10) {
                sum061 += workloadPerformance[i][j];
                counter061++;
                sum062 += workloadPerformance[i][j + 2];
                counter062++;
            }
        }
        averagePerformances[0] = sum01 / counter01;
        averagePerformances[1] = sum02 / counter02;
        averagePerformances[2] = sum031 / counter031;
        averagePerformances[3] = sum032 / counter032;
        averagePerformances[4] = sum033 / counter033;
        averagePerformances[5] = sum034 / counter034;
        averagePerformances[6] = sum041 / counter041;
        averagePerformances[7] = sum042 / counter042;
        averagePerformances[8] = sum05 / counter05;
        averagePerformances[9] = sum061 / counter061;
        averagePerformances[10] = sum062 / counter062;

        return averagePerformances;
    }

    private static void setOverallPerformance (int n, int t) {
        int [] averagePerformances = averagePerformances();
        for (int i = 0; i < 11; i++) {
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

        for (int i = 0; i < 11; i ++) {
            chartName = "LineChart: Query" + queryNames[i] + " bei konstantem n";
            LinechartGenerator linechartGenerator = new LinechartGenerator(xValuesTime, overallPerformance[i][0], xNameTime, yName, chartName);
            linechartGenerator.createChart();
        }

        for (int i = 0; i < 11; i ++) {
            chartName = "LineChart: Query" + queryNames[i] + " bei konstantem t";
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

        overallPerformance = new int [11][8][8];

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
