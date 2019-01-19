import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import { BeatLoader } from 'react-spinners';

// Props:
// stimmkreis=Number [optional]: if given, shows the result only for one Stimmkreis; if omitted, shows the result for the whole election.
// filter=(v => true/false) [optional]: applies a filter to the parties shown.
export default class Stimmverteilung extends Component {
  constructor(props) {
    super(props);
    this.updateData = this.updateData.bind(this);

    this.state = {
      time: 0,
      isLoaded2013: false,
      isLoaded2018: false,
      stimmverteilung2018: [],
      stimmverteilung2013: [],
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
    if (this.state.isLoaded2013 && this.state.isLoaded2018) {
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
                    label: '2013 (links)',
                    data: this.state.stimmverteilung2018.map(v18 => (this.state.stimmverteilung2013.find(v13 => v13.PARTEI === v18.PARTEI) || { PROZENT: 0 }).PROZENT),
                    backgroundColor: this.state.stimmverteilung2018.map(v18 => '#' + v18.PARTEI_FARBE),
                  },
                  {
                    label: '2018 (rechts)',
                    data: this.state.stimmverteilung2018.map(v18 => v18.PROZENT),
                    backgroundColor: this.state.stimmverteilung2018.map(v18 => '#' + v18.PARTEI_FARBE),
                  },
                ],
                labels: this.state.stimmverteilung2018.map(v => v.PARTEI),
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

    fetch(`/api/stimmverteilung/${this.props.stimmkreis >= 109 && this.props.stimmkreis < 200 ? this.props.stimmkreis - 1 : this.props.stimmkreis || ''}/2013${(this.props.stimmkreis)? `?compute_on_aggregated_date=${this.props.computeOnAggregatedData}` : ''}`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ stimmverteilung2013: data });
        this.setState({ isLoaded2013: true });
      });
    fetch(`/api/stimmverteilung/${this.props.stimmkreis || ''}/2018${(this.props.stimmkreis)? `?compute_on_aggregated_date=${this.props.computeOnAggregatedData}` : ''}`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ stimmverteilung2018: this.props.filter ? data.filter(v => this.props.filter(v)) : data });
        this.setState({ isLoaded2018: true });
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
