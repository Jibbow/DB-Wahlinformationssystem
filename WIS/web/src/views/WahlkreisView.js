import React, { Component } from 'react';
import BayernMap from '../components/BayernMap';

export default class WahlkreisView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      parteien: [],
      ueberhangmandate: {
        Oberbayern: [],
        Niederbayern: [],
        Oberpfalz: [],
        Oberfranken: [],
        Mittelfranken: [],
        Unterfranken: [],
        Schwaben: [],
      },
    };
  }

  render() {
    return (
      <div className="two-column-tab-content">
        <BayernMap mode={'wk'} onClick={x => console.log(x)} />
        <div>
          <h2>Überhangmandate</h2>
          {Object.keys(this.state.ueberhangmandate).map(k => (
            <div  key={k + "div"}>
              <h3  id={k + "headers"}>{k}</h3>
              <table class="table"  id={k.partei + "table"}>
                <thead>
                  <tr>
                    <th scope="col">Partei</th>
                    <th scope="col">Überhangmandate</th>
                  </tr>
                </thead>
                <tbody>
                  {Object.keys(this.state.ueberhangmandate[k])
                    .map(k2 => this.state.ueberhangmandate[k][k2])
                    .map(v => (
                      <tr key={v.partei}>
                        <td id={v.partei + "p"}>{v.partei}</td>
                        <td id={v.partei + "ueber"}>{v.ueberhangmandate}</td>
                      </tr>
                    ))}
                </tbody>
              </table>
            </div>
          ))}
        </div>
      </div>
    );
  }

  componentDidMount() {
    let start = performance.now();
    fetch('http://localhost:8000/parteien')
      .then(response => response.json())
      .then(data => {
        this.state.parteien = data;
      })
      .then(() => {
        for (const x of Array(7).keys()) {
          for (const p in this.state.parteien) {
            const url = "http://localhost:8000/ueberhangmandate/" + (x + 1)
                        + "/" +  p + "/2018";
            fetch(url)
              .then(response => {
                return response.json()}
              )
              .then(data => {
                const man = this.state.ueberhangmandate;
                const arr = man[data.WAHLKREIS];
                arr.push({
                  partei: data.PARTEI,
                  ueberhangmandate: data.UEBERHANGMANDATE
                })
                this.forceUpdate();
              })
          }
        }
      })
  }
}
