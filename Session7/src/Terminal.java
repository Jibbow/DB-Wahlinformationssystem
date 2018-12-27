import java.io.IOException;
import java.io.InputStream;
import java.net.URL;



public class Terminal implements Runnable {

    private int meanWaitTime;
    private int noRequests;
    private String [] workloadMix;
    private volatile Integer [] workloadPerformance;
    private int workloadPointer;

    public Terminal (int meanWaitTime, int noRequests, String [] workloadMix, int workloadPointer) {
        super();
        this.meanWaitTime = meanWaitTime;
        this.noRequests = noRequests;
        this.workloadMix = workloadMix;
        this.workloadPerformance = new Integer [noRequests];
        this.workloadPointer = workloadPointer;
    }

    private int getWaitTime () {
        double minimum = meanWaitTime * 0.8;
        double maximum = meanWaitTime * 1.2;
        double range = maximum - minimum;
        double random = Math.random();
        random = random * range;
        int waitTime = (int) (random + minimum);
        System.out.println("Waiting " + waitTime + "ms");
        return waitTime;
    }

    @Override
    public void run () {
        for (int i = 0; i < noRequests; i++) {
            String url = workloadMix[workloadPointer];

            try {
                long startTime = System.currentTimeMillis();
                new URL(url).openStream();
                int duration = (int) (System.currentTimeMillis() - startTime);
                workloadPerformance[workloadPointer] = duration;
            } catch (IOException e) {
                e.printStackTrace();
            }


            workloadPointer = (workloadPointer + 1) % noRequests;

            int waitTime = getWaitTime();
            try {
                Thread.sleep(waitTime);
            }
            catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

    }

    public Integer [] getWorkloadPerformance () {
        return this.workloadPerformance;
    }
}
