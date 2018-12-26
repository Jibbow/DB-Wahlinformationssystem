import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import { BeatLoader } from 'react-spinners';

export default class StimmentwicklungStimmkreis extends Component {
  constructor(props) {
    super(props);
    
    this.state = {
      time: 0,
      stimmentwicklung: [],
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
                    data: this.state.stimmentwicklung.map(v => v.DIFF_PROZENT),
                    backgroundColor: this.state.stimmentwicklung.map(v => v.FARBE),
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

  componentDidUpdate(prevProps, prevState) {
    if (this.props.stimmkreis !== prevProps.stimmkreis) {
      let start = performance.now();
      fetch(`http://localhost:8000/stimmverteilungdifferenz/${this.props.stimmkreis}`)
        .then(response => response.json())
        .then(data => {
          let end = performance.now();
          this.setState({ time: end - start });
          this.setState({ stimmentwicklung: data });
        });
    }
  }
}