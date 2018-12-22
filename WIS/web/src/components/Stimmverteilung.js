import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import { BeatLoader } from 'react-spinners';

// Props:
// stimmkreis=Number [optional]: if given, shows the result only for one Stimmkreis; if omitted, shows the result for the whole election.
// filter=(v => true/false) [optional]: applies a filter to the parties shown.
export default class Stimmverteilung extends Component {
  constructor(props) {
    super(props);
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
                    backgroundColor: this.state.stimmverteilung2018.map(v18 => v18.FARBE),
                  },
                  {
                    label: '2018 (rechts)',
                    data: this.state.stimmverteilung2018.map(v18 => v18.PROZENT),
                    backgroundColor: this.state.stimmverteilung2018.map(v18 => v18.FARBE),
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

  componentDidMount() {
    let start = performance.now();
    fetch(`http://localhost:8000/stimmverteilung/${this.props.stimmkreis || ''}/2013`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ stimmverteilung2013: data });
        this.setState({ isLoaded2013: true });
      });
    fetch(`http://localhost:8000/stimmverteilung/${this.props.stimmkreis || ''}/2018`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ stimmverteilung2018: this.props.filter ? data.filter(v => this.props.filter(v)) : data });
        this.setState({ isLoaded2018: true });
      });
  }
}
