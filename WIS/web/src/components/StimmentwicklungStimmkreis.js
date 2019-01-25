import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import { BeatLoader } from 'react-spinners';

export default class StimmentwicklungStimmkreis extends Component {
  constructor(props) {
    super(props);
    this.updateData = this.updateData.bind(this);
    
    this.state = {
      time: 0,
      stimmentwicklung: [],
      chartoptions: {
        legend: {
          position: 'top',
          labels: {
            boxWidth: 0,
            fontSize: 18,
          },
        },
        scales: {
          xAxes: [
            {
              categoryPercentage: 0.5,
            },
          ],
        },
      },
    };
  }

  render() {
    if (this.state.time !== 0) {
      return (
        <div>
          <small className="text-muted">Took {this.state.time} milliseconds</small>
          <div className="chart">
            <Bar
              className="chart"
              options={this.state.chartoptions}
              data={{
                datasets: [
                  {
                    label: "Stimmentwicklung der Parteien",
                    data: this.state.stimmentwicklung.map(v => v.DIFF_PROZENT),
                    backgroundColor: this.state.stimmentwicklung.map(v => '#' + v.PARTEI_FARBE),
                  },
                ],
                labels: this.state.stimmentwicklung.map(v => v.PARTEI),
              }}
            />
          </div>
        </div>
      );
    } else {
      return (
        <div>
          <div className="spinner">
            <BeatLoader color={'#93dee2'} />
          </div>
        </div>
      );
    }
  }

  updateData() {
    let start = performance.now();
    fetch(`http://localhost:8000/stimmverteilungdifferenz/${this.props.stimmkreis}?compute_on_aggregated_data=${this.props.computeOnAggregatedData}`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ stimmentwicklung: data });
      });
  }

  componentDidUpdate(prevProps, prevState) {
    if (this.props.stimmkreis !== prevProps.stimmkreis) {
      this.updateData();
    }
  }

  componentDidMount() {
    this.updateData();
  }
}
