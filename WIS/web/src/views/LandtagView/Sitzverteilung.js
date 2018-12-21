import React, { Component } from 'react';
import { Doughnut } from 'react-chartjs-2';

export default class Sitzverteilung extends Component {
  constructor(props) {
    super(props);
    this.state = {
      time: 0,
      isLoaded: false,
      sitzverteilung: [],
      chartoptions: {
        rotation: 1 * Math.PI,
        circumference: 1 * Math.PI,
      },
    };
  }

  render() {
    if (this.state.isLoaded) {
      return (
        <div>
          <small class="text-muted">Took {this.state.time} milliseconds</small>
          <Doughnut
            width={500}
            options={this.state.chartoptions}
            data={{
              datasets: [
                {
                  data: this.state.sitzverteilung.map(v => v.SITZE),
                  backgroundColor: this.state.sitzverteilung.map(v => '#3e95cd'),
                },
              ],
              labels: this.state.sitzverteilung.map(v => v.PARTEI),
            }}
          />
          <table class="table">
            <thead>
              <tr>
                <th scope="col">Partei</th>
                <th scope="col">Sitze</th>
              </tr>
            </thead>
            <tbody>
              {this.state.sitzverteilung.map(v => (
                <tr key={'sitzverteilung-tr-' + v.PARTEI}>
                  <td key={'sitzverteilung-td-partei-' + v.PARTEI}>{v.PARTEI}</td>
                  <td key={'sitzverteilung-td-sitze-' + v.PARTEI}>{v.SITZE}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      );
    } else {
      return (
        <div>
          <small class="text-muted">Waiting for results...</small>
        </div>
      );
    }
  }

  componentDidMount() {
    let start = performance.now();
    fetch('http://localhost:8000/sitzverteilung/2018')
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ sitzverteilung: data });
        this.setState({ isLoaded: true });
      });
  }
}
