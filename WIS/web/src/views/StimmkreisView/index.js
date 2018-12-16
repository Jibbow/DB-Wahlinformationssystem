import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import BayernMap from '../../components/BayernMap';

export default class StimmkreisView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      stimmkreis: 0,
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
      parteiergebnisdifferenz: {
        data: {
          datasets: [
            {
              label: 'Änderung',
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
      <div className="row">
        <div className="col-xs-6">
          <BayernMap mode={'sk'} onClick={x => this.setState({ stimmkreis: x })} />
        </div>
        <div className="col-xs-6">
          {this.state.stimmkreis !== 0 && (
            <div>
              <h2>Stimmkreis {this.state.stimmkreis}</h2>
              <h3>Gewählter Direktkandidat: {this.state.gewinner.VORNAME + ' ' + this.state.gewinner.NACHNAME + ' ' + this.state.gewinner.PARTEI}</h3>
              {this.state.gewinner.time !== 0 && <small className="text-muted">Took {this.state.gewinner.time} milliseconds</small>}
              <h3>Verteilung der Stimmen</h3>
              {this.state.parteiergebnis.time !== 0 && <small className="text-muted">Took {this.state.parteiergebnis.time} milliseconds</small>}
              <Bar width={400} data={this.state.parteiergebnis.data} options={this.state.parteiergebnis.options} />
              <h3>Prozentuale Änderung der Stimmen im Vergleich zu 2013</h3>
              {this.state.parteiergebnisdifferenz.time !== 0 && <small className="text-muted">Took {this.state.parteiergebnisdifferenz.time} milliseconds</small>}
              <Bar width={400} data={this.state.parteiergebnisdifferenz.data} options={this.state.parteiergebnisdifferenz.options} />
            </div>
          )}
        </div>
      </div>
    );
  }

  componentDidUpdate(prevProps, prevState) {
    if (this.state.stimmkreis !== prevState.stimmkreis) {
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
      fetch(`http://localhost:8000/parteiergebnisdifferenz/${this.state.stimmkreis}`)
        .then(response => response.json())
        .then(data => {
          let end = performance.now();
          this.state.parteiergebnisdifferenz.time = end - start;
          this.state.parteiergebnisdifferenz.data.labels = data.map(v => v.PARTEI);
          this.state.parteiergebnisdifferenz.data.datasets[0].data = data.map(v => v.DIFF_PROZENT);
          this.forceUpdate();
        });
    }
  }
}
