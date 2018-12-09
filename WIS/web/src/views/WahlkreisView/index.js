import React, { Component } from 'react';
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';


export default class WahlkreisView extends Component {
    constructor(props) {
    super(props);
    this.state = {
        ueberhangmandate: {
            Oberbayern: [],
            Niederbayern: [],
            Oberpfalz: [],
            Oberfranken: [],
            Mittelfranken: [],
            Unterfranken: [],
            Schwaben: [],
        }
    };
  }

    render() {
        return (
            <div class="row">
                <div class="col-xs-6">
                    <img src={bayern_map} className="bayern-map" alt="Karte von Bayern" />
                </div>
                <div class="col-xs-6">
                    <h2>Überhangmandate</h2>
                    {
                        Object.keys(this.state.ueberhangmandate).map(k =>
                            <div>
                                <h3>{k}</h3>
                                <table class="table">
                                    <thead>
                                        <tr>
                                        <th scope="col">Partei</th>
                                        <th scope="col">Überhangmandate</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            Object.keys(this.state.ueberhangmandate[k])
                                                .map(k2 => this.state.ueberhangmandate[k][k2])
                                                .map(v => <tr><td>{v.partei}</td><td>{v.ueberhangmandate}</td></tr>)
                                        }
                                    </tbody>
                                </table>
                            </div>
                        )
                    }
                    

                </div>
            </div>
        );
    }

    componentDidMount() {
        let start = performance.now();
        fetch('http://localhost:8000/ueberhangmandate/2018')
            .then(response => response.json())
            .then(data => {
                let end = performance.now();
                this.state.ueberhangmandate.time = end - start;
                data.forEach(v => 
                    this.state.ueberhangmandate[v.WAHLKREIS].push(
                            {
                                partei: v.PARTEI,
                                ueberhangmandate: v.UEBERHANGMANDATE
                            }
                        )
                );
                this.forceUpdate();
            });
    }
}
