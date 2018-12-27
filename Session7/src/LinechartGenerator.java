import org.jfree.chart.ChartPanel;
import org.jfree.chart.ChartFactory;
import org.jfree.chart.JFreeChart;
import org.jfree.ui.ApplicationFrame;
import org.jfree.ui.RefineryUtilities;
import org.jfree.chart.plot.PlotOrientation;
import org.jfree.data.category.DefaultCategoryDataset;

public class LinechartGenerator extends ApplicationFrame {

    private int [] xValues;
    private int [] yValues;
    private String xName;
    private String yName;
    private String chartName;

    public LinechartGenerator (int [] xValues, int [] yValues, String xName, String yName, String chartName) {
        super("Performanz-Evaluation");
        this.xValues = xValues;
        this.yValues = yValues;
        this.xName = xName;
        this.yName = yName;
        this.chartName = chartName;
    }

    private DefaultCategoryDataset createDataset() {
        DefaultCategoryDataset dataset = new DefaultCategoryDataset( );
        for (int i = 0; i < xValues.length; i++) {
            dataset.addValue(yValues[i], "Performanz", Integer.toString(xValues[i]));
        }
        return dataset;
    }

    public void createChart () {
        JFreeChart lineChart = ChartFactory.createLineChart(
                chartName,
                xName,yName,
                createDataset(),
                PlotOrientation.VERTICAL,
                true,true,false);

        ChartPanel chartPanel = new ChartPanel( lineChart );
        chartPanel.setPreferredSize( new java.awt.Dimension( 560 , 367 ) );
        setContentPane( chartPanel );

        this.pack( );
        RefineryUtilities.centerFrameOnScreen(this);
        this.setVisible( true );



    }


}
