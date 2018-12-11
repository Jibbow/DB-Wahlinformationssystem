import React, { Component } from 'react';
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
  }
}
