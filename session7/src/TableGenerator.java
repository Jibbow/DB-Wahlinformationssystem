import org.jfree.chart.ChartUtilities;

import javax.swing.JFrame;
import javax.swing.JScrollPane;
import javax.swing.JTable;
import javax.swing.SwingUtilities;
import java.io.File;
import java.io.IOException;

public class TableGenerator extends JFrame {

    String [] columns;
    Integer [][] workloadPerformance;

    public TableGenerator (String [] columns, Integer [][] workloadPerformance) {
        super();
        this.columns = columns;
        this.workloadPerformance = workloadPerformance;
    }

    public void createTable () {
        JTable table = new JTable(workloadPerformance, columns);
        this.add(new JScrollPane(table));

        this.setTitle("Performance Evaluation");
        this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        this.pack();
        this.setVisible(true);


    }
}
