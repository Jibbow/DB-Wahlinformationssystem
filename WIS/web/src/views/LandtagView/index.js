import React, { Component } from 'react';
import { Doughnut, Bar } from 'react-chartjs-2';
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';


export default class LandtagView extends Component {
    constructor(props) {
    super(props);
    this.state = {
      sitzverteilung: {
          data: {
              datasets: [
                  {
                      data: [10, 20, 30],
                      backgroundColor: ["#3e95cd", "#8e5ea2","#3cba9f"],
                  }
              ],
              labels: [
                  '...',
                  '...',
                  '...'
              ]
          },
          options: {
            rotation: 1 * Math.PI,
            circumference: 1 * Math.PI
          }
      },
      stimmverteilung: {
          data: {
              datasets: [
                  {
                      label: "2018",
                      data: [20.5, 13.2, 5.0],
                      backgroundColor: ["#3e95cd", "#8e5ea2","#3cba9f"],
                  },
                  {
                      label: "2013",
                      data: [40.5, 15.2, 3.4],
                      backgroundColor: ["#3e95cd", "#8e5ea2","#3cba9f"],
                  }
              ],
              labels: [
                  '...',
                  '...',
                  '...'
              ]
          },
          options: {

          }
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
                    
                    <h2>Knappste Sieger in den Stimmkreisen</h2>
                    sdf
                </div>
            </div>
        );
    }

    componentDidMount() {
        fetch('http://localhost:8000/stimmverteilung/2018')
            .then(response => response.json())
            .then(data => {
                this.state.stimmverteilung.data.labels = data.map(v => v.PARTEI);
                this.state.stimmverteilung.data.datasets[0].data = data.map(v => v.PROZENT);
                this.forceUpdate();
            });
        fetch('http://localhost:8000/sitzverteilung/2018')
            .then(response => response.json())
            .then(data => {
                this.state.sitzverteilung.data.labels = data.map(v => v.PARTEI);
                this.state.stimmverteilung.data.datasets[0].data = data.map(v => v.SITZE);
                this.forceUpdate();
            });
    }
}
