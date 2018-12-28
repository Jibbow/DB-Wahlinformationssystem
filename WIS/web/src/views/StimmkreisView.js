import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import BayernMap from '../components/BayernMap';
import Stimmverteilung from '../components/Stimmverteilung';
import StimmentwicklungStimmkreis from '../components/StimmentwicklungStimmkreis';
import StimmenAbsolut from '../components/StimmenAbsolut';

export default class StimmkreisView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      stimmkreis: 0,
      stimmkreise: [],
      gewinner: {
        time: 0,
        person: {},
      },
      wahlbeteiligung: {
        time: 0,
        value: 0.0,
      },
      siegerparteierststimmen: {
        time: 0,
        partei: '',
      },
      siegerparteizweitstimmen: {
        time: 0,
        partei: '',
      },
    };
  }

  render() {
    return (
      <div className="two-column-tab-content">
        <BayernMap mode={'sk'} onClick={x => this.setState({ stimmkreis: x })} />
        <div>
          {(this.state.stimmkreis !== 0 && (
            <div>
              {this.state.stimmkreise && (
                <div>
                  <h1>{this.state.stimmkreise.find(s => s.NR === this.state.stimmkreis).NAME}</h1>
                  <p>Stimmkreis: {this.state.stimmkreise.find(s => s.NR === this.state.stimmkreis).NR}</p>
                  <p>Wahlkreis: {this.state.stimmkreise.find(s => s.NR === this.state.stimmkreis).WAHLKREIS}</p>
                  <p>Wahlberechtigte: {this.state.stimmkreise.find(s => s.NR === this.state.stimmkreis).STIMMBERECHTIGTE}</p>
                </div>
              )}
              <div>
                <h4>Gewählter Direktkandidat: {this.state.gewinner.person.VORNAME + ' ' + this.state.gewinner.person.NACHNAME + ' (' + this.state.gewinner.person.PARTEI + ')'}</h4>
                {this.state.gewinner.time !== 0 && <small className="text-muted">Took {this.state.gewinner.time} milliseconds</small>}
              </div>
              <div>
                <h4>Siegerpartei nach Erststimmen: {this.state.siegerparteierststimmen.partei}</h4>
                {this.state.siegerparteierststimmen.time !== 0 && <small className="text-muted">Took {this.state.siegerparteierststimmen.time} milliseconds</small>}
                <h4>Siegerpartei nach Zweitstimmen: {this.state.siegerparteizweitstimmen.partei}</h4>
                {this.state.siegerparteizweitstimmen.time !== 0 && <small className="text-muted">Took {this.state.siegerparteizweitstimmen.time} milliseconds</small>}
              </div>
              <div>
                <h4>Wahlbeteiligung: {this.state.wahlbeteiligung.value} %</h4>
                {this.state.wahlbeteiligung.time !== 0 && <small className="text-muted">Took {this.state.wahlbeteiligung.time} milliseconds</small>}
              </div>
              <div>
                <h3>Verteilung der Stimmen</h3>
                <Stimmverteilung stimmkreis={this.state.stimmkreis} /*filter={v => v.PROZENT >= 5.0}*/ />
              </div>
              <div>
                <h3>Entwicklung der Stimmen im Vergleich zu 2013</h3>
                <StimmentwicklungStimmkreis stimmkreis={this.state.stimmkreis} />
              </div>
              <div>
                <h3>Anzahl an Stimmen für jede Partei</h3>
                <StimmenAbsolut stimmkreis={this.state.stimmkreis} />
              </div>
            </div>
          )) || <p>Wählen Sie links auf der Karte einen Stimmkreis</p>}
        </div>
      </div>
    );
  }

  componentDidMount() {
    fetch(`http://localhost:8000/stimmkreise/2018`)
      .then(response => response.json())
      .then(data => {
        this.setState({ stimmkreise: data });
        console.log(this.state.stimmkreise);
      });
  }

  componentDidUpdate(prevProps, prevState) {
    if (this.state.stimmkreis !== prevState.stimmkreis) {
      let start = performance.now();
      fetch(`http://localhost:8000/direktkandidatengewinner/${this.state.stimmkreis}/2018`)
        .then(response => response.json())
        .then(data => {
          let end = performance.now();
          this.setState({ gewinner: { time: end - start, person: data } });
        });
      fetch(`http://localhost:8000/wahlbeteiligung/${this.state.stimmkreis}/2018`)
        .then(response => response.json())
        .then(data => {
          let end = performance.now();
          this.setState({ wahlbeteiligung: { time: end - start, value: data.WAHLBETEILIGUNG } });
        });
      fetch(`http://localhost:8000/siegerpartei/erststimmen/${this.state.stimmkreis}/2018`)
        .then(response => response.json())
        .then(data => {
          let end = performance.now();
          this.setState({ siegerparteierststimmen: { time: end - start, partei: data.PARTEI } });
        });
      fetch(`http://localhost:8000/siegerpartei/zweitstimmen/${this.state.stimmkreis}/2018`)
        .then(response => response.json())
        .then(data => {
          let end = performance.now();
          this.setState({ siegerparteizweitstimmen: { time: end - start, partei: data.PARTEI } });
        });
    }
  }
}
