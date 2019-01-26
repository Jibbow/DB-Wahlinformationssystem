import React, { Component } from 'react';
import { Button, Collapse } from 'react-bootstrap';
import BayernMap from '../components/BayernMap';
import SitzverteilungLandtag from '../components/SitzverteilungLandtag';
import Stimmverteilung from '../components/Stimmverteilung';

export default class LandtagView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      landtagsmitglieder: {
        data: [],
        time: 0,
      },
    };
  }

  render() {
    return (
      <div className="two-column-tab-content">
        <BayernMap mode={'none'} onClick={x => console.log(x)} />
        <div>
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
    fetch('/api/landtagsmitglieder/2018')
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
