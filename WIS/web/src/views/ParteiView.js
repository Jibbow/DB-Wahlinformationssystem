import React, { Component } from 'react';
import { DropdownButton, MenuItem } from 'react-bootstrap';
import { Table } from 'react-bootstrap';
import bayern_map from '../assets/Bayern_Landtagswahlkreise_2018.svg';
import WikipediaInfo from '../components/WikipediaInfo';

export default class ParteiView extends Component {
  constructor(props) {
    super(props);
    this.state = {
      parteien: [],
      selectedParteiId: 0,
      sieger: []
    };
  }

  render() {
    return (
      <div className="two-column-tab-content">
        <img src={bayern_map} className="bayern-map" alt="Karte von Bayern" />
        <div>
          <DropdownButton title={'Wähle eine Partei'} id={'dropdown-parteien'}
            onSelect={(key, event) => {
              this.setState({ selectedParteiId: key })
              this.fetchSieger(key);
              }
            }>
            {this.state.parteien.map(p => (
              <MenuItem key={p.ID} eventKey={p.ID}>
                {p.NAME}
              </MenuItem>
            ))}
          </DropdownButton>
          {this.state.selectedParteiId !== 0 &&
            <WikipediaInfo title={this.state.parteien.find(p => p.ID === this.state.selectedParteiId).NAME}/>
          }
          
          <h2>Knappste Gewinner / Verlierer</h2>
          <Table>
            <thead>
              <tr>
                <th scope="col">Platzierung</th>
                <th scope="col">Name</th>
                <th scope="col">Vorsprung</th>
              </tr>
            </thead>
            <tbody>
              {this.state.sieger.map(v => (
                <tr key={'knappste-tr-' + v.PLATZIERUNG}>
                  <td key={'knappste-td-platz-' + v.PLATZIERUNG}>{v.PLATZIERUNG}</td>
                  <td key={'knappste-td-name-' + v.PLATZIERUNG}>{v.VORNAME} {v.NACHNAME}</td>
                  <td key={'knappste-td-diff-' + v.DIFFERENZ}>{v.DIFFERENZ}</td>
                </tr>
              ))}
            </tbody>
          </Table>
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
        this.forceUpdate();
      });
      this.fetchSieger(1);
  }

  fetchSieger(id) {
    if (id === 1 || id < this.state.parteien.length) {
      fetch('http://localhost:8000/knappstesieger/' + id + '/2018')
        .then(response => response.json())
        .then(data => {
          this.setState({sieger: data});
          this.forceUpdate();
        })
    }
  }
}
