import React, { Component } from 'react';
import { Scatter } from 'react-chartjs-2';
import bayern_map from '../assets/Bayern_Landtagswahlkreise_2018.svg';

export default class AnalysisView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      csu_sterberate: {
        data: {
          datasets: [
            {
              data: [0, 0],
              backgroundColor: ['#3e95cd'],
            },
          ],
          labels: ['...'],
        },
        options: {},
        time: 0,
      },
      fdp_einkommen: {
        data: {
          datasets: [
            {
              data: [0, 0],
              backgroundColor: ['#3e95cd'],
            },
          ],
          labels: ['...'],
        },
        options: {},
        time: 0,
      },
    };
  }

  render() {
    return (
      <div class="panel">
        <h2>FDP-Wähler - Einkommen</h2>
        <Scatter width={600} data={this.state.fdp_einkommen.data} options={this.state.fdp_einkommen.options} />
        <h2>CSU-Wähler - Sterberate</h2>
        <Scatter width={600} data={this.state.csu_sterberate.data} options={this.state.csu_sterberate.options} />
      </div>
    );
  }

  componentDidMount() {
    let start = performance.now();
    fetch('http://localhost:8000//analysen/fdp-einkommen')
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.fdp_einkommen.time = end - start;
        this.state.fdp_einkommen.data.datasets[0].data = data.map(v => {
          return {
            x: v.EINKOMMEN,
            y: v.PROZENT,
          };
        });
        this.forceUpdate();
      });
    fetch('http://localhost:8000//analysen/csu-sterberate')
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.csu_sterberate.time = end - start;
        this.state.csu_sterberate.data.datasets[0].data = data.map(v => {
          return {
            x: v.STERBERATE,
            y: v.PROZENT,
          };
        });
        this.forceUpdate();
      });
  }
}
