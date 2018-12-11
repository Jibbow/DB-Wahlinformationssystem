import React, { Component } from 'react';
import { Doughnut, Bar } from 'react-chartjs-2';
import { Button, Collapse } from 'react-bootstrap'
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';


export default class LandtagView extends Component {
    constructor(props) {
    super(props);
    this.state = {
      sitzverteilung: {
          data: {
              datasets: [
                  {
                      data: [0],
                      backgroundColor: ["#3e95cd"],
                  }
              ],
              labels: [
                  '...',
              ]
          },
          options: {
            rotation: 1 * Math.PI,
            circumference: 1 * Math.PI
          },
          time: 0
      },
      stimmverteilung: {
          data: {
              datasets: [
                  {
                      label: "2018",
                      data: [20.5, 13.2, 5.0],
                      backgroundColor: ["#3e95cd"],
                  },
                  {
                      label: "2013",
                      data: [],
                      backgroundColor: ["#3e95cd"],
                  }
              ],
              labels: [
                  '...',
                  '...',
                  '...'
              ]
          },
          options: {

          },
          time: 0
      },
      landtagsmitglieder: {
          data: [],
          time: 0
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
                    <h2>Sitzverteilung im Landtag</h2>
                    {this.state.sitzverteilung.time !== 0 &&
                        <small class="text-muted">Took {this.state.sitzverteilung.time} milliseconds</small>   
                    }
                    <Doughnut width={600} data={this.state.sitzverteilung.data} options={this.state.sitzverteilung.options}/>
                    <h4>Für die Landtagswahl ergab sich folgende Verteilung der Sitze an die Parteien:</h4>
                    <table class="table">
                        <thead>
                            <tr>
                            <th scope="col">Partei</th>
                            <th scope="col">Sitze</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                this.state.sitzverteilung.data.labels
                                    .map((v,i) => { return { partei: v, sitze: this.state.sitzverteilung.data.datasets[0].data[i] } })
                                    .map(t => <tr><td>{t.partei}</td><td>{t.sitze}</td></tr>)
                            }
                        </tbody>
                    </table>

                    <h2>Ergebnisse der Parteien im Vergleich</h2>
                    <Bar width={600} data={this.state.stimmverteilung.data} options={this.state.sitzverteilung.options}/>

                    <h2>Mitglieder im Landtag</h2>
                    <Button onClick={() => this.setState({ open: !this.state.open })}>
                        Mitglieder im Landtag {!this.state.open && 'anzeigen' || 'verbergen'}
                    </Button>
                    <div>
                    {this.state.landtagsmitglieder.time !== 0 &&
                        <small class="text-muted">Took {this.state.landtagsmitglieder.time} milliseconds</small>   
                    }
                    </div>
                    <Collapse in={this.state.open}>
                        <div>
                            <table class="table">
                                <thead>
                                    <tr>
                                    <th scope="col">Name</th>
                                    <th scope="col">Partei</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {
                                        this.state.landtagsmitglieder.data
                                            .map(v => <tr><td>{v.name}</td><td>{v.partei}</td></tr>)
                                    }
                                </tbody>
                            </table>
                        </div>
                    </Collapse>
 
                    <h2>Knappste Sieger in den Stimmkreisen</h2>
                    sdf
                </div>
            </div>
        );
    }

    componentDidMount() {
        let start = performance.now();
        fetch('http://localhost:8000/stimmverteilunggesamt/2018')
            .then(response => response.json())
            .then(data => {
                let end = performance.now();
                this.state.stimmverteilung.time = end - start;
                this.state.stimmverteilung.data.labels = data.map(v => v.PARTEI);
                this.state.stimmverteilung.data.datasets[0].data = data.map(v => v.PROZENT);
                this.forceUpdate();
            });
        fetch('http://localhost:8000/sitzverteilung/2018')
            .then(response => response.json())
            .then(data => {
                let end = performance.now();
                this.state.sitzverteilung.time = end - start;
                this.state.sitzverteilung.data.labels = data.map(v => v.PARTEI);
                this.state.sitzverteilung.data.datasets[0].data = data.map(v => v.SITZE);
                this.forceUpdate();
            });
        fetch('http://localhost:8000/landtagsmitglieder/2018')
            .then(response => response.json())
            .then(data => {
                let end = performance.now();
                this.state.landtagsmitglieder.time = end - start;
                this.state.landtagsmitglieder.data = data.map(v => {
                    return {
                        name: v.VORNAME + ' ' + v.NACHNAME,
                        partei: v.PARTEI
                    }
                });
                this.forceUpdate();
            });
    }
}
