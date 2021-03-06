import React, { Component } from 'react';
import { Doughnut } from 'react-chartjs-2';
import { Table } from 'react-bootstrap';
import { BeatLoader } from 'react-spinners';

export default class SitzverteilungLandtag extends Component {
  constructor(props) {
    super(props);
    this.state = {
      time: 0,
      isLoaded: false,
      sitzverteilung: [],
      chartoptions: {
        rotation: 1 * Math.PI,
        circumference: 1 * Math.PI,
        legend: {
          position: 'right',
        },
      },
    };
  }

  render() {
    if (this.state.isLoaded) {
      return (
        <div>
          <small className="text-muted">Took {this.state.time} milliseconds</small>
          <h4>Gesamtzahl an Sitzen: {this.state.sitzverteilung.reduce((acc, c) => acc + c.SITZE, 0)}</h4>
          <div className="chart">
            <Doughnut
              className="chart"
              options={this.state.chartoptions}
              data={{
                datasets: [
                  {
                    data: this.state.sitzverteilung.map(v => v.SITZE),
                    backgroundColor: this.state.sitzverteilung.map(v => '#' + v.PARTEI_FARBE),
                  },
                ],
                labels: this.state.sitzverteilung.map(v => v.PARTEI),
              }}
            />
          </div>
          <Table>
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
          </Table>
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
    fetch(`/api/sitzverteilung/2018`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ sitzverteilung: data });
        this.setState({ isLoaded: true });
      });
  }
}
