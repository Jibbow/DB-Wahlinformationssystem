import React, { Component } from 'react';
import { Bar } from 'react-chartjs-2';
import BayernMap from '../components/BayernMap';
import Stimmverteilung from '../components/Stimmverteilung';

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
    };
  }

  render() {
    return (
      <div className="two-column-tab-content">
        <BayernMap mode={'sk'} onClick={x => this.setState({ stimmkreis: x })} />
        <div>
          {this.state.stimmkreis !== 0 && (
            <div>
              <h2>Stimmkreis {this.state.stimmkreis}</h2>
              <h3>Gewählter Direktkandidat: {this.state.gewinner.VORNAME + ' ' + this.state.gewinner.NACHNAME + ' (' + this.state.gewinner.PARTEI + ')'}</h3>
              {this.state.gewinner.time !== 0 && <small className="text-muted">Took {this.state.gewinner.time} milliseconds</small>}
              <h3>Verteilung der Stimmen</h3>
              <Stimmverteilung stimmkreis={this.state.stimmkreis} filter={v => v.PROZENT >= 5.0}/>
            </div>
          ) || <p>Wählen Sie links auf der Karte einen Stimmkreis</p>}
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
          this.state.gewinner = data;
          this.state.gewinner.time = end - start;
          this.forceUpdate();
        });
    }
  }
}
