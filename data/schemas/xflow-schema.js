export default {
  '$schema': 'http://nosuchtype.com/xflow/v1/schema#',
  'id': '/',
  'type': 'object',
  'properties': {
    'id': {
      'id': 'id',
      'type': 'string',
      'minLength': 1
    },
    'name': {
      'id': 'name',
      'type': 'string',
      'minLength': 1
    },
    'version': {
      'id': 'version',
      'type': 'number',
      'minimum': 0
    },
    'requirements': {
      'id': 'requirements',
      'type': 'array',
      'minItems': 1,
      'uniqueItems': true,
      'additionalItems': true,
      'items': {
        'type': 'object',
        'properties': {
          'xtype': {
            'id': 'xtype',
            'type': 'string',
            'minLength': 1
          },
          'version': {
            'id': 'version',
            'type': 'integer',
            'minimum': 1,
            'default': 1
          }
        },
        'additionalProperties': false
      }
    },
    'variables': {
      'id': 'variables',
      'type': 'object',
      'properties': {
        'input': {
          'type': 'array',
          'minItems': 0,
          'uniqueItems': true,
          'additionalItems': true,
          'items': {
            'type' : 'object',
            'oneOf' : [
              { '$ref': '#/definitions/scopeVariable' }
            ]
          }
        },
        'output': {
          'type': 'array',
          'minItems': 0,
          'uniqueItems': true,
          'additionalItems': true,
          'items': {
            'type': 'object',
            'oneOf' : [
              { '$ref': '#/definitions/scopeVariableReference' }
            ]
          }
        },
        'local': {
          'id': 'local',
          'type': 'array',
          'minItems': 0,
          'uniqueItems': true,
          'additionalItems': true,
          'items': {
            'type': 'object',
            'oneOf' : [
              { '$ref': '#/definitions/scopeVariable' }
            ]
          }
        }
      },
      'additionalProperties': false,
      'required' : [
        'input',
        'output',
        'local'
      ]
    },
    'nodes': {
      'id': 'nodes',
      'type': 'array',
      'minItems': 2,
      'uniqueItems': true,
      'additionalItems': true,
      'items': {
        'type': 'object',
        'oneOf' : [
          { '$ref': '#/definitions/flowNode' }
        ]
      }
    },
    'edges': {
      'id': 'edges',
      'type': 'array',
      'minItems': 1,
      'uniqueItems': true,
      'additionalItems': true,
      'items': {
        'type': 'array',
        'oneOf' : [
          { '$ref': '#/definitions/flowEdge' }
        ]
      }
    },
    'branches': {
      'id': 'branches',
      'type': 'array',
      'minItems': 0,
      'uniqueItems': true,
      'additionalItems': true,
      'items': {
        'type': 'object',
        'oneOf' : [
          { '$ref': '#/definitions/flowBranch' }
        ]
      }
    }
  },
  'definitions' : {
    'scopeVariableReference' : {
      'properties' : {
        'name' : {
          'type': 'string',
          'minLength': 1
        },
        'vtype' : {
          'type': 'string',
          'minLength': 1,
          'default': 'string',
          'enum': [
            'boolean',
            'number',
            'string'
          ]
        }
      },
      'additionalProperties' : false,
      'required': [
        'name',
        'vtype'
      ]
    },
    'scopeVariable' : {
      'properties' : {
        'name' : {
          'type': 'string',
          'minLength': 1
        },
        'vtype' : {
          'type': 'string',
          'minLength': 1,
          'default': 'string',
          'enum': [
            'boolean',
            'number',
            'string'
          ]
        },
        'value' : {
          'type': 'string',
          'minLength': 0,
          'default': ''
        }
      },
      'additionalProperties' : false,
      'required': [
        'name',
        'vtype',
        'value'
      ]
    },
    'flowNode' : {
      'properties' : {
        'id' : {
          'type' : 'number'
        },
        'nodetype' : {
          'type' : 'string'
        },
        'action' : {
          'type' : 'string'
        },
        'label' : {
          'type' : 'string'
        },
        'parameters' : {
          'type' : 'object'
        }
      },
      'additionalProperties' : false,
      'required': [
        'id',
        'nodetype',
        'action',
        'label',
        'parameters'
      ]
    },
    'flowEdge' : {
      'properties' : {
        'type': 'array',
        'minItems': 2,
        'maxItems': 2,
        'uniqueItems': true,
        'additionalItems': false,
        'items': {
          'type': 'number'
        },
        'additionalProperties': false
      }
    },
    'flowBranch' : {
      'properties' : {
        'edge' : {
          'type': 'object',
          'oneOf' : [
            { '$ref': '#/definitions/flowEdge' }
          ]
        },
        'name' : {
          'type' : 'string'
        },
        'value' : {
          'type': 'object',
          'anyOf': [
            { 'type': 'boolean' },
            { 'type': 'string' },
            { 'type': 'number' }
          ]
        }
      },
      'additionalProperties' : false,
      'required': [
        'edge',
        'name',
        'value'
      ]
    }
  },
  'additionalProperties': false,
  'required': [
    'id',
    'name',
    'version',
    'requirements',
    'variables',
    'nodes',
    'edges',
    'branches'
  ]
};



