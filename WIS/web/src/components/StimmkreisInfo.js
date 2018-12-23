import React, { Component } from 'react';
import { BeatLoader } from 'react-spinners';
import { Panel, Table } from 'react-bootstrap';

// Props:
// stimmkreis=Number
export default class StimmkreisInfo extends Component {
  constructor(props) {
    super(props);
    //this.updateData = this.updateData.bind(this);

    this.state = {};
  }

  render() {
    return (
      <div>
        <Panel>
          <Panel.Heading>Allgemeine Informationen <small className="text-muted">({this.state.time} milliseconds)</small></Panel.Heading>
          <Table>
            <tbody>
              <tr>
                <td>Name</td>
                <td>München-Hadern</td>
              </tr>
              <tr>
                <td>Nummer</td>
                <td>101</td>
              </tr>
              <tr>
                <td>Wahlkreis</td>
                <td>Oberbayern</td>
              </tr>
              <tr>
                <td>Wahlbeteiligung</td>
                <td>72,2%</td>
              </tr>
            </tbody>
          </Table>
        </Panel>
        <Panel>
          <Panel.Heading>Landtagsabgeordneter <small className="text-muted">({this.state.time} milliseconds)</small></Panel.Heading>
          <Table>
            <tbody>
              <tr>
                <td>Name</td>
                <td>Georg Eisenreich</td>
              </tr>
              <tr>
                <td>Partei</td>
                <td>CSU</td>
              </tr>
              <tr>
                <td>Stimmanteil</td>
                <td>29,1%</td>
              </tr>
            </tbody>
          </Table>
        </Panel>
      </div>
    );
  }
}
