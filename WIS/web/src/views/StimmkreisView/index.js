import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';

export default class StimmkreisView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      stimmkreis: 101,
      gewinner: {
        ID: 0,
        VORNAME: '',
        NACHNAME: '',
        PARTEI: '',
        time: 0,
      },
      parteiergebnis: {
        data: {
          datasets: [
            {
              label: '2018',
              data: [0],
              backgroundColor: ['#3e95cd'],
            },
          ],
          labels: ['...'],
        },
        options: {},
        time: 0,
      },
      parteiergebnis2013: {
        data: {
          datasets: [
            {
              label: '2013',
              data: [0],
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
      <div class="row">
        <div class="col-xs-6">
          <img src={bayern_map} className="bayern-map" alt="Karte von Bayern" />
        </div>
        <div class="col-xs-6">
          <h2>Stimmkreis {this.state.stimmkreis}</h2>
          <h3>Gewählter Direktkandidat: {this.state.gewinner.VORNAME + ' ' + this.state.gewinner.NACHNAME + ' ' + this.state.gewinner.PARTEI}</h3>
          {this.state.gewinner.time !== 0 && <small class="text-muted">Took {this.state.gewinner.time} milliseconds</small>}
          <h3>Verteilung der Stimmen</h3>
          {this.state.parteiergebnis.time !== 0 && <small class="text-muted">Took {this.state.parteiergebnis.time} milliseconds</small>}
          <Bar width={400} data={this.state.parteiergebnis.data} options={this.state.parteiergebnis.options} />
          <Bar width={400} data={this.state.parteiergebnis2013.data} options={this.state.parteiergebnis2013.options} />
        </div>
      </div>
    );
  }

  componentDidMount() {
    let start = performance.now();
    fetch(`http://localhost:8000/direktkandidatengewinner/${this.state.stimmkreis}/2018`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.gewinner = data[0];
        this.state.gewinner.time = end - start;
        this.forceUpdate();
      });
    fetch(`http://localhost:8000/parteiergebnis/${this.state.stimmkreis}/2018`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.parteiergebnis.time = end - start;
        this.state.parteiergebnis.data.labels = data.map(v => v.PARTEI);
        this.state.parteiergebnis.data.datasets[0].data = data.map(v => v.STIMMENRELATIV);
        this.forceUpdate();
      });
      fetch(`http://localhost:8000/parteiergebnis/${this.state.stimmkreis}/2013`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.parteiergebnis2013.time = end - start;
        this.state.parteiergebnis2013.data.labels = data.map(v => v.PARTEI);
        this.state.parteiergebnis2013.data.datasets[0].data = data.map(v => v.STIMMENRELATIV);
        this.forceUpdate();
      });
  }
}
