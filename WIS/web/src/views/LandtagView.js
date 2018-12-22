import React, { Component } from 'react';
import { Doughnut, Bar } from 'react-chartjs-2';
import { Button, Collapse } from 'react-bootstrap';
import BayernMap from '../components/BayernMap';
import SitzverteilungLandtag from '../components/SitzverteilungLandtag';
import Stimmverteilung from '../components/Stimmverteilung';

export default class LandtagView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      stimmverteilung: {
        data: {
          datasets: [
            {
              label: '2018',
              data: [20.5, 13.2, 5.0],
              backgroundColor: ['#3e95cd'],
            },
            {
              label: '2013',
              data: [],
              backgroundColor: ['#3e95cd'],
            },
          ],
          labels: ['...', '...', '...'],
        },
        options: {},
        time: 0,
      },
      landtagsmitglieder: {
        data: [],
        time: 0,
      },
    };
  }

  render() {
    return (
      <div class="row">
        <div class="col-xs-6">
          <BayernMap mode={'none'} onClick={x => console.log(x)} />
        </div>
        <div class="col-xs-6">
          <h2>Sitzverteilung im Landtag</h2>
          <SitzverteilungLandtag/>
          

          <h2>Ergebnisse der Parteien im Vergleich</h2>
          <Stimmverteilung filter={v => v.PROZENT >= 5.0}/>
          
          <h2>Mitglieder im Landtag</h2>
          <Button onClick={() => this.setState({ open: !this.state.open })}>Mitglieder im Landtag {(!this.state.open && 'anzeigen') || 'verbergen'}</Button>
          <div>{this.state.landtagsmitglieder.time !== 0 && <small class="text-muted">Took {this.state.landtagsmitglieder.time} milliseconds</small>}</div>
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
                  {this.state.landtagsmitglieder.data.map(v => (
                    <tr>
                      <td>{v.name}</td>
                      <td>{v.partei}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </Collapse>
        </div>
      </div>
    );
  }

  componentDidMount() {
    let start = performance.now();
    /*fetch('http://localhost:8000/stimmverteilung/2018')
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.stimmverteilung.time = end - start;
        this.state.stimmverteilung.data.labels = data.map(v => v.PARTEI);
        this.state.stimmverteilung.data.datasets[0].data = data.map(v => v.PROZENT);
        this.forceUpdate();
      });*/
    fetch('http://localhost:8000/landtagsmitglieder/2018')
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.state.landtagsmitglieder.time = end - start;
        this.state.landtagsmitglieder.data = data.map(v => {
          return {
            name: v.VORNAME + ' ' + v.NACHNAME,
            partei: v.PARTEI,
          };
        });
        this.forceUpdate();
      });
  }
}
